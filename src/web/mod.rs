use rocket::{Rocket, Build};

mod boring;
mod weekly;

use boring::{add_routes as add_boring_routes};
use weekly::{add_routes as add_weekly_routes};

pub(crate) fn add_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    let mut r = add_boring_routes(rocket);
    r = add_weekly_routes(r);
    r

}