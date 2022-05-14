use rocket::{form::Form, {get, launch, routes, post, FromForm}, response::{Flash, Redirect}, http::{CookieJar, Cookie}, uri, serde::Serialize};
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct Context {
    name: Option<String>,
    is_admin: bool,
    logged_in: bool,
}

impl Context {
    pub fn from_cookiejar(cookies: &CookieJar<'_>) -> Self {
        Self {
            name: cookies.get_private("name").map_or(None, |x| Some(x.value().to_owned())),
            is_admin: cookies.get_private("is_admin").map_or(false, |x| x.value().parse().unwrap()),
            logged_in: cookies.get_private("logged_in").map_or(false, |x| x.value().parse().unwrap()),
        }
    }
}

#[get("/login")]
fn login(cookies: &CookieJar<'_>) -> Template {
    Template::render("login", Context::from_cookiejar(cookies))
}

#[derive(FromForm)]
#[allow(unused)]
struct LoginForm<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
}

#[post("/login", data = "<form>")]
fn login_post(form: Form<LoginForm<'_>>, cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.add_private(Cookie::new("logged_in", "true"));
    cookies.add_private(Cookie::new("name", form.name.to_owned()));
    if form.name.trim() == "admin" {
        cookies.add_private(Cookie::new("is_admin", "true"));
    }

    Flash::success(Redirect::to(uri!("/")), "Successfully logged in")
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("logged_in"));
    cookies.remove_private(Cookie::named("name"));
    cookies.remove_private(Cookie::named("is_admin"));
    Flash::success(Redirect::to(uri!("/")), "Successfully logged out")
}

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
    Template::render("index", Context::from_cookiejar(cookies))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, login, login_post, logout])
}
