use crate::rules::{get_weekly_ruleset, most_recent_sunday, TemplateState};
use crate::techniques::{Ruleset, RulesetTemplate, TECHNIQUE_NAMES};
use chrono::{Date, Datelike, Month, NaiveDate, TimeZone, Utc};
use rand::rngs::SmallRng;
use rocket::http::ext::IntoCollection;
use rocket::response::status::NotFound;
use rocket::{get, Build, Rocket};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use std::fs::File;
use std::path::{Path, PathBuf};

fn month_from_u32(w: u32) -> Month {
    match w {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => {
            panic!("Illegal month")
        }
    }
}

#[derive(Serialize)]
struct WeeklyRuleset<'a> {
    week_of: String,
    ruleset: &'a Ruleset,
    technique_names: &'static [&'static str],
    active_tab: String,
}

#[derive(Serialize)]
struct SerializedRuleset {
    day: i32,
    name: String,
    ruleset: Ruleset,
    technique_names: Vec<String>,
}

#[derive(Deserialize)]
struct DeserializedRuleset {
    day: i32,
    name: String,
    ruleset: HashMap<String, String>,
    technique_names: Vec<String>,
}

fn saved_rulesets_root_path() -> PathBuf {
    PathBuf::from(&var("RULESETS_PATH").unwrap_or("rulesets".to_string()))
}

fn saved_weeklies_path() -> PathBuf {
    let mut p = saved_rulesets_root_path();
    p.push("weeklies");
    p
}

fn save_weekly(ruleset: Ruleset, date: &Date<Utc>) -> Result<(), String> {
    let mut path = saved_weeklies_path();
    std::fs::create_dir_all(path.clone()).map_err(|e| format!("Error creating paths: {}", e))?;
    path.push(format!("{}.json", date.num_days_from_ce()));
    if path.exists() {
        return Err("Serialized weekly already exists!".to_string());
    }
    let f = File::create(path).map_err(|e| e.to_string())?;

    let sw = SerializedRuleset {
        day: date.num_days_from_ce(),
        name: day_to_nice_string(date),
        ruleset,
        technique_names: TECHNIQUE_NAMES.iter().map(|s| s.to_string()).collect(),
    };
    serde_json::to_writer(f, &sw).map_err(|e| e.to_string())
}

fn day_to_nice_string(day: &Date<Utc>) -> String {
    format!(
        "{} {}, {}",
        month_from_u32(day.month()).name(),
        day.day(),
        day.year()
    )
}

fn get_saved_ruleset<P: AsRef<Path>>(path: P) -> Result<DeserializedRuleset, String> {
    let f = File::open(path).map_err(|e| e.to_string())?;

    serde_json::from_reader(f).map_err(|e| format!("Deserialization error: {}", e))
}

// at some point this should probably not involve a ton of disk IO and parsing on-demand, lol
fn get_saved_weeklies() -> Vec<DeserializedRuleset> {
    let p = saved_weeklies_path();
    let files = std::fs::read_dir(p)
        .map(|rd| {
            rd.filter_map(|rde| rde.ok().map(|de| de.path()))
                .collect::<Vec<PathBuf>>()
        })
        .unwrap_or(vec![]);
    let mut deserialized = vec![];
    for file in files {
        match get_saved_ruleset(file) {
            Ok(r) => {
                deserialized.push(r);
            }
            Err(e) => {
                println!("Error getting ruleset: {}", e);
                continue;
            }
        }
    }
    deserialized
}

#[derive(Serialize)]
struct SavedInfo {
    display_name: String,
    id: i32,
}

#[get("/history/<id>")]
async fn render_past_ruleset(id: i32) -> Result<Template, NotFound<String>> {
    #[derive(Serialize)]
    struct Ctx {
        active_tab: String,
        ruleset: HashMap<String, String>,
        technique_names: Vec<String>,
        name: String,
    }

    let mut p = saved_weeklies_path();
    p.push(format!("{}.json", id));
    let r = get_saved_ruleset(p).map_err(|s| NotFound(s))?;
    println!("Rendering ruleset");
    Ok(Template::render(
        "historical_ruleset",
        Ctx {
            active_tab: "history".to_string(),
            ruleset: r.ruleset,
            name: r.name,
            technique_names: r.technique_names,
        },
    ))
}

#[get("/history")]
async fn history() -> Template {
    #[derive(Serialize)]
    struct Ctx {
        files: Vec<SavedInfo>,
        active_tab: String,
    }

    let mut rulesets: Vec<SavedInfo> = get_saved_weeklies()
        .iter()
        .map(|ds| SavedInfo {
            display_name: ds.name.clone(),
            id: ds.day,
        })
        .collect();
    rulesets.sort_by_key(|r| r.id);

    Template::render(
        "history",
        Ctx {
            files: rulesets,
            active_tab: "history".to_string(),
        },
    )
}

#[get("/weekly")]
async fn weekly() -> Template {
    let now = chrono::offset::Utc::now();
    let last_sunday = most_recent_sunday(now.date());
    let r = get_weekly_ruleset();
    let rc = WeeklyRuleset {
        week_of: day_to_nice_string(&last_sunday),
        ruleset: &r,
        technique_names: &TECHNIQUE_NAMES,
        active_tab: "weekly".to_string(),
    };
    match save_weekly(r.clone(), &last_sunday) {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving weekly: {}", e);
        }
    }

    Template::render("weekly_ruleset", rc)
}

pub(crate) fn add_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", rocket::routes![weekly, history, render_past_ruleset])
}
