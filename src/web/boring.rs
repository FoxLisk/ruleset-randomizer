//! Boring webpages lol. info/about/whatever

use rocket::fs::NamedFile;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, get, Rocket, Build};
use std::path::{Path, PathBuf};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use serde::Serialize;
use crate::rules::IsAllowed;

const STATIC_SUFFIXES: [&str; 8] = [
    &"js", &"css", &"png", &"mp3", &"html", &"jpg", &"ttf", &"otf",
];

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
    let rules = vec![
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
        (
            "Arbitrary Code Execution".to_string(),
            IsAllowed::DISALLOWED,
        ),
    ];
    let c = SupplementalContext {
        active_tab: "supplemental".to_string(),
        rules,
    };
    Template::render("supplemental", c)
}


pub(crate) fn add_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", rocket::routes![favicon, about, supplemental])
        .mount("/static", rocket::routes![statics])
}


