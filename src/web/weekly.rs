use crate::rules::{get_weekly_ruleset, most_recent_sunday};
use crate::techniques::{Ruleset,  TECHNIQUE_NAMES};
use chrono::{Date, Datelike, Month, Utc};
use rocket::response::status::NotFound;
use rocket::{get, Build, Rocket, State};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use std::fs::File;
use std::path::{Path, PathBuf};
use sqlx::{SqlitePool, Row};

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
    ruleset: Ruleset,
    technique_names: Vec<String>,
}

#[derive(Deserialize)]
struct DeserializedRuleset {
    day: i32,
    ruleset: HashMap<String, String>,
    technique_names: Vec<String>,
}

fn saved_rulesets_root_path() -> PathBuf {
    PathBuf::from(&var("RULESETS_PATH").unwrap_or("rulesets".to_string()))
}

fn saved_weeklies_dir() -> PathBuf {
    let mut p = saved_rulesets_root_path();
    p.push("weeklies");
    p
}

async fn save_weekly(ruleset: Ruleset, date: &Date<Utc>, pool: &SqlitePool) -> Result<(), String> {
    // save to disk
    let mut path = saved_weeklies_dir();
    std::fs::create_dir_all(path.clone()).map_err(|e| format!("Error creating paths: {}", e))?;
    path.push(format!("{}.json", date.num_days_from_ce()));
    if path.exists() {
        return Err("Serialized weekly already exists!".to_string());
    }
    let f = File::create(&path).map_err(|e| e.to_string())?;
    let name = day_to_nice_string(date);

    let sw = SerializedRuleset {
        day: date.num_days_from_ce(),
        ruleset,
        technique_names: TECHNIQUE_NAMES.iter().map(|s| s.to_string()).collect(),
    };
    serde_json::to_writer(f, &sw).map_err(|e| e.to_string())?;

    // save to db
    sqlx::query(
        "INSERT INTO rulesets (id, name, filename) VALUES (?, ?, ?)")
        .bind(date.num_days_from_ce())
        .bind(name)
        .bind(path.file_name().unwrap().to_string_lossy().into_owned())
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
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


#[derive(Serialize)]
struct SavedInfo {
    display_name: String,
    id: i32,
}

#[derive(sqlx::FromRow, Serialize)]
struct RulesetRecord {
    id: u32,
    name: String,
    filename: String,
}

#[get("/history/<id>")]
async fn render_past_ruleset(id: u32, pool: &State<SqlitePool>) -> Result<Template, NotFound<String>> {
    #[derive(Serialize)]
    struct Ctx {
        active_tab: String,
        ruleset: HashMap<String, String>,
        technique_names: Vec<String>,
        name: String,
    }

    let rec: RulesetRecord = sqlx::query_as("SELECT id, name, filename FROM rulesets WHERE id = ?")
        .bind(id)
        .fetch_one(&**pool)
        .await
        .map_err(|e|
            {
                println!("Error fetching ruleset: {:?}", e);
                NotFound("Unknown ruleset id".to_string())
            }
        )?;



    let mut p = saved_weeklies_dir();
    p.push(rec.filename);
    let r = get_saved_ruleset(p).map_err(|s| NotFound(s))?;
    Ok(Template::render(
        "historical_ruleset",
        Ctx {
            active_tab: "history".to_string(),
            ruleset: r.ruleset,
            name: rec.name,
            technique_names: r.technique_names,
        },
    ))
}

#[get("/history")]
async fn history(pool: &State<SqlitePool>) -> Template {
    #[derive(Serialize)]
    struct Ctx {
        rulesets: Vec<RulesetRecord>,
        active_tab: String,
    }

    let mut rulesets = match sqlx::query_as(
        "SELECT id, name, filename FROM rulesets")
        .fetch_all(&**pool)
        .await {
        Ok(o) => o,
        Err(e) => {
            println!("Error fetching rulesets: {:?}", e);
            vec![]
        }
    };
    Template::render(
        "history",
        Ctx {
            rulesets,
            active_tab: "history".to_string(),
        },
    )
}

#[get("/weekly")]
async fn weekly(pool: &State<SqlitePool>) -> Template {
    let now = chrono::offset::Utc::now();
    let last_sunday = most_recent_sunday(now.date());
    let r = get_weekly_ruleset();
    let rc = WeeklyRuleset {
        week_of: day_to_nice_string(&last_sunday),
        ruleset: &r,
        technique_names: &TECHNIQUE_NAMES,
        active_tab: "weekly".to_string(),
    };
    match save_weekly(r.clone(), &last_sunday, &pool).await {
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
