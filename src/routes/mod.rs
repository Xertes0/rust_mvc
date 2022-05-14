use rocket::http::CookieJar;
use serde::Serialize;

pub mod index;
pub mod auth;

#[derive(Serialize)]
struct UserContext {
    name: Option<String>,
    is_admin: bool,
    logged_in: bool,
}

impl UserContext {
    pub fn from_cookiejar(cookies: &CookieJar<'_>) -> Self {
        Self {
            name: cookies.get_private("name").map_or(None, |x| Some(x.value().to_owned())),
            is_admin: cookies.get_private("is_admin").map_or(false, |x| x.value().parse().unwrap()),
            logged_in: cookies.get_private("logged_in").map_or(false, |x| x.value().parse().unwrap()),
        }
    }
}
