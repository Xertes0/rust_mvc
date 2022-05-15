use rocket::{get, Route, routes};
use rocket_dyn_templates::{Template, context};

use crate::guards::UserGuard;

#[get("/")]
fn index(user: Option<UserGuard>) -> Template {
    // TODO don't clone
    Template::render("index", context!{ user: user.map(|x| (*x).clone()) })
}

pub fn get_routes() -> Vec<Route> {
    routes![index]
}
