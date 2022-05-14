use rocket::{http::CookieJar, get, Route, routes};
use rocket_dyn_templates::{Template, context};

use super::UserContext;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
    Template::render("index", context!{ user: UserContext::from_cookiejar(cookies) })
}

pub fn get_routes() -> Vec<Route> {
    routes![index]
}
