mod rules;
mod techniques;
mod web;

use crate::rules::{InputWeights, IsAllowed, MGRules, NMGRules, NoEGRules, RMGRules, TemplateState, get_weekly_ruleset};
use crate::techniques::{Ruleset, RulesetTemplate, TECHNIQUE_NAMES};
use chrono::{Date, DateTime, Datelike, Month, Offset, TimeZone, Weekday};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rocket::form::{Form, FromForm};
use rocket::fs::{NamedFile, TempFile};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{get, post};
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::Template;
use serde::Serialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::migrate::{MigrateError, Migrator};


#[derive(Serialize)]
struct EmptyContext {}

const EMPTY_CONTEXT: EmptyContext = EmptyContext {};

// TODO: combine these static-ish-pages routes into one


#[get("/upload")]
async fn upload_form() -> Template {
    Template::render("submit_weights", EMPTY_CONTEXT)
}

#[get("/")]
async fn root() -> Redirect {
    // can't use the `weekly` function b/c it's not in scope and i guess i dont really want to make it in scope?
    Redirect::to(rocket::uri!("/weekly"))
}

#[derive(Serialize)]
struct RulesetContext<'a> {
    ruleset: &'a Ruleset,
}

#[derive(FromForm)]
struct Upload<'f> {
    upload: TempFile<'f>,
}

#[post("/upload", data = "<form>")]
async fn upload(mut form: Form<Upload<'_>>) {
    println!("{:?}", form.upload);
    let c = match &form.upload {
        TempFile::File { path, .. } => read_to_string(path).unwrap(),
        TempFile::Buffered { content } => content.to_string(),
    };
    // form.upload.persist_to("uploads/blah").await;
    println!("Uploaded file contents: {}", c);
}

#[get("/comparisons")]
async fn comparisons() -> Json<Vec<Ruleset>> {
    Json(vec![
        NMGRules.clone(),
        RMGRules.clone(),
        NoEGRules.clone(),
        MGRules.clone(),
        get_weekly_ruleset(),
    ])
}

#[get("/world")]
fn hello() -> String {
    "Hello, world!".to_string()
}

fn build_rocket() -> Rocket<Build> {
    let mut r = rocket::build()
        .mount(
            "/",
            rocket::routes![
                hello,
                // upload_form,
                // upload,
                root,
                comparisons,
            ],
        )
        .attach(Template::fairing());
    r = web::add_routes(r);
    r
}

async fn get_pool() -> Result<SqlitePool, sqlx::Error> {
    let sqlite_db_path = std::env::var("DATABASE_PATH").unwrap_or("db/test.db3".to_string());
    let p = Path::new(&sqlite_db_path);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    // use a SqliteConnectOptions instead of a hardcoded queryparam?
    let path_with_params = format!("sqlite://{}?mode=rwc", sqlite_db_path);
    SqlitePoolOptions::new()
        .max_connections(12)
        .connect(&path_with_params)
        .await
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), MigrateError> {
    let migrator = Migrator::new(Path::new("migrations")).await?;
    migrator.run(pool).await
}

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    let mut t = InputWeights {
        name: "hi".to_string(),
        defaults: "NMGRules".to_string(),
        weights: Default::default(),
    };

    t.weights
        .insert("FakeFlippers".to_string(), "false".to_string());
    println!("{}", serde_yaml::to_string(&t).unwrap());

    let pool = get_pool().await.unwrap();
    match run_migrations(&pool).await {
        Ok(_) => {},
        Err(e) => {
            println!("Migration error: {:?}", e);
            panic!();
        }
    }
    let rocket = build_rocket()
        .manage(pool);
    let ignited = rocket.ignite().await.unwrap();
    ignited.launch().await.unwrap();
}
