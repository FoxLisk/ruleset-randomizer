mod rules;
mod techniques;

use rocket::{Rocket, Build, Request};
use rocket_dyn_templates::Template;
use std::path::{PathBuf, Path};
use rocket::fs::{NamedFile, TempFile};
use rocket::request::{FromRequest, Outcome};
use rocket::{get, post};
use serde_yaml;
use crate::rules::{InputWeights, IsAllowed, NMGRules, RMGRules, NoEGRules, MGRules, TemplateState};
use std::fs::read_to_string;
use rocket::form::{Form, FromForm};
use serde::Serialize;
use crate::techniques::{Ruleset, RulesetTemplate, TECHNIQUE_NAMES};
use rand::rngs::SmallRng;
use chrono::{DateTime, Offset, TimeZone, Datelike, Date, Weekday, Month};
use rand::SeedableRng;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use std::collections::HashMap;

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
        _ => {panic!("Illegal month")}
    }
}


const STATIC_SUFFIXES: [&str; 8] = [&"js", &"css", &"png", &"mp3", &"html", &"jpg", &"ttf", &"otf"];


#[derive(Serialize)]
struct EmptyContext {}

const EMPTY_CONTEXT: EmptyContext = EmptyContext {};

struct StaticAsset {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaticAsset {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let path = request.uri().path();
        let filename = match path.segments().last() {
            Some(f) => f,
            None => return Outcome::Failure((rocket::http::Status::NotFound, ())),
        };
        let suffix = filename.rsplit('.').next().unwrap();
        if STATIC_SUFFIXES.contains(&suffix) {
            Outcome::Success(StaticAsset {})
        } else {
            Outcome::Failure((rocket::http::Status::NotFound, ()))
        }
    }
}

#[get("/<file..>")]
async fn statics(file: PathBuf, _asset: StaticAsset) -> Option<NamedFile> {
    let p = Path::new("static/").join(file);
    if !p.exists() {
        println!("{:?} does not exist", p);
        return None;
    }
    NamedFile::open(p).await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").await.ok()

}

// TODO: combine these static-ish-pages routes into one

#[get("/about")]
async fn about() -> Template {
    let mut context: HashMap<String, String> = Default::default();
    context.insert("active_tab".to_string(), "about".to_string());
    Template::render("about", context)
}

#[derive(Serialize)]
struct SupplementalContext {
    active_tab: String,
    rules: Vec<(String, IsAllowed)>,
}

#[get("/supplemental")]
async fn supplemental() -> Template {
    let rules  = vec![
        ("Spooky Action".to_string(), IsAllowed::ALLOWED),
        ("Torch Glitch".to_string(), IsAllowed::ALLOWED),
        ("Block Clips".to_string(), IsAllowed::ALLOWED),
        ("Big Bomb Dupe".to_string(), IsAllowed::ALLOWED),
        ("Water Walk".to_string(), IsAllowed::ALLOWED),
        ("Houlihan".to_string(), IsAllowed::ALLOWED),
        ("Medallion Cancel".to_string(), IsAllowed::ALLOWED),

        ("Super Bunny".to_string(), IsAllowed::ALLOWED),
        ("Dungeon Revival".to_string(), IsAllowed::ALLOWED),
        ("Surfing Bunny".to_string(), IsAllowed::ALLOWED),
        ("Bunny Pocket".to_string(), IsAllowed::ALLOWED),
        ("UnBunnyBeam".to_string(), IsAllowed::ALLOWED),

        ("Arbitrary Code Execution".to_string(), IsAllowed::DISALLOWED),

    ];
    let c = SupplementalContext {
        active_tab: "supplemental".to_string(),
        rules,
    };
    Template::render("supplemental", c)
}


#[get("/upload")]
async fn upload_form() -> Template {
    Template::render("submit_weights", EMPTY_CONTEXT)
}

#[get("/")]
async fn root() -> Redirect {
    Redirect::to(rocket::uri!(weekly))
}

#[derive(Serialize)]
struct RulesetContext<'a> {
    ruleset: &'a Ruleset,
}

fn most_recent_sunday<tz: TimeZone>(mut d: Date<tz>) -> Date<tz> {
    while d.weekday() != chrono::Weekday::Sun {
        d = d.pred();
    }
    d
}

#[derive(Serialize)]
struct WeeklyRuleset<'a> {
    week_of: String,
    ruleset: &'a Ruleset,
    technique_names: &'static [&'static str],
    active_tab: String,
}

fn get_weekly_ruleset() -> Ruleset {
    let rt = RulesetTemplate {
        SaveAndQuit: TemplateState::CHANCE_PER_THOUSAND(200),
        FakeFlippers: TemplateState::CHANCE_PER_THOUSAND(980),
        BombJump: TemplateState::CHANCE_PER_THOUSAND(980),
        SilverlessGanon: TemplateState::CHANCE_PER_THOUSAND(990),
        ItemDash: TemplateState::CHANCE_PER_THOUSAND(950),
        AncillaOverload: TemplateState::CHANCE_PER_THOUSAND(950),
        Hover: TemplateState::CHANCE_PER_THOUSAND(850),
        HammerJump: TemplateState::CHANCE_PER_THOUSAND(980),
        DoorStateExtension: TemplateState::CHANCE_PER_THOUSAND(330),
        DiverDown: TemplateState::CHANCE_PER_THOUSAND(330),
        OverworldBunnyRevival: TemplateState::CHANCE_PER_THOUSAND(800),
        HeraPot: TemplateState::CHANCE_PER_THOUSAND(200),
        OverworldClipping: TemplateState::CHANCE_PER_THOUSAND(100),
        OverworldMirrorGlitches: TemplateState::CHANCE_PER_THOUSAND(100),
        OverworldYBA: TemplateState::CHANCE_PER_THOUSAND(100),
        SuperSpeed: TemplateState::CHANCE_PER_THOUSAND(950),
        OverworldEG: TemplateState::CHANCE_PER_THOUSAND(50),
        Misslotting: TemplateState::CHANCE_PER_THOUSAND(50),
        HookShopping: TemplateState::CHANCE_PER_THOUSAND(100),
        OverworldSwimmyG: TemplateState::CHANCE_PER_THOUSAND(100),
        UnderworldClipping: TemplateState::CHANCE_PER_THOUSAND(50),
        UnderworldYBA: TemplateState::CHANCE_PER_THOUSAND(30),
        UnderworldDeathHole: TemplateState::CHANCE_PER_THOUSAND(30),
        SomariaTransitionCorruption: TemplateState::CHANCE_PER_THOUSAND(30),
        DoorJukes: TemplateState::CHANCE_PER_THOUSAND(20),
        LayerDisparity: TemplateState::STATIC(IsAllowed::DISALLOWED),
    };
    let now = chrono::offset::Utc::now();
    let last_sunday = most_recent_sunday(now.date());

    let mut rng = SmallRng::seed_from_u64(1 + last_sunday.num_days_from_ce() as u64);
    let mut r =rt.apply_with_rng(&NMGRules, &mut rng);
    r.name = "Weekly".to_string();
    r
}

#[get("/weekly")]
async fn weekly() -> Template {

    let now = chrono::offset::Utc::now();
    let last_sunday = most_recent_sunday(now.date());
    let r = get_weekly_ruleset();
    let rc = WeeklyRuleset {
        week_of: format!("{} {}, {}", month_from_u32(last_sunday.month()).name(), last_sunday.day(), last_sunday.year()),
        ruleset: &r,
        technique_names: &TECHNIQUE_NAMES,
        active_tab: "weekly".to_string()
    };


    Template::render("weekly_ruleset", rc)
}


#[derive(FromForm)]
struct Upload<'f> {
    upload: TempFile<'f>
}

#[post("/upload", data = "<form>")]
async fn upload(mut form: Form<Upload<'_>>)  {
    println!("{:?}", form.upload);
    let c = match &form.upload {
        TempFile::File { path, .. } => {

            read_to_string(path).unwrap()
        }
        TempFile::Buffered { content } => {
            content.to_string()
        }
    };
    // form.upload.persist_to("uploads/blah").await;
    println!("Uploaded file contents: {}", c);
}

#[get("/comparisons")]
async fn comparisons() -> Json<Vec<Ruleset>> {
    Json(vec!(NMGRules.clone(), RMGRules.clone(), NoEGRules.clone(), MGRules.clone(), get_weekly_ruleset()))

}

#[get("/world")]
fn hello() -> String {
    "Hello, world!".to_string()
}

fn build_rocket(
) -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
            hello,
            // upload_form,
            // upload,
            weekly,
            root,
            comparisons,
            about,
            supplemental,
            favicon],
        )
        .mount("/static", rocket::routes![statics])
        .attach(Template::fairing())
}


#[rocket::main]
async fn main() {
    println!("Hello, world!");
    let mut t = InputWeights {
        name: "hi".to_string(),
        defaults: "NMGRules".to_string(),
        weights: Default::default()
    };

    t.weights.insert("FakeFlippers".to_string(), "false".to_string());
    println!("{}", serde_yaml::to_string(&t).unwrap());

    let rocket = build_rocket();
    let ignited = rocket.ignite().await.unwrap();
    ignited.launch().await.unwrap();
}

