mod rules;
mod techniques;

use rocket::{Rocket, Build, Request};
use rocket_dyn_templates::Template;
use std::path::{PathBuf, Path};
use rocket::fs::{NamedFile, TempFile};
use rocket::request::{FromRequest, Outcome};
use rocket::{get, post};
use serde_yaml;
use crate::rules::{InputWeights, IsAllowed};
use std::fs::read_to_string;
use rocket::form::{Form, FromForm};
use serde::Serialize;
use crate::techniques::Ruleset;


const STATIC_SUFFIXES: [&str; 7] = [&"js", &"css", &"mp3", &"html", &"jpg", &"ttf", &"otf"];

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

#[get("/upload")]
async fn upload_form() -> Template {
    Template::render("submit_weights", EMPTY_CONTEXT)
}

#[derive(Serialize)]
struct RulesetContext<'a> {
    ruleset: &'a Ruleset,
}

#[get("/ruleset")]
async fn show_ruleset() -> Template {
    let r = Ruleset {
        FakeFlippers: IsAllowed::ALLOWED,
        OverworldClipping: IsAllowed::DISALLOWED,
    };
    let rc = RulesetContext {
        ruleset: &r,
    };
    Template::render("ruleset", rc)
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


#[get("/world")]
fn hello() -> String {
    "Hello, world!".to_string()
}

fn build_rocket(
) -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![hello, upload_form, upload, show_ruleset],
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

