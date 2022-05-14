use diesel::{RunQueryDsl, dsl::sql, QueryDsl, ExpressionMethods, result::Error::NotFound};
use rocket::{form::Form, {FromForm, http::{CookieJar, Cookie}, response::{Flash, Redirect}, uri, get, post}, Route, routes};
use rocket_dyn_templates::{Template, context};
use sha2::{Sha256, Digest};

use crate::{db_pool::DbPool, models::{User, NewUser}, schema::users};
use super::UserContext;

#[derive(FromForm)]
struct LoginForm<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(FromForm)]
struct RegisterForm<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
}

// Login

#[get("/login?<failed>")]
fn login(cookies: &CookieJar<'_>, failed: Option<bool>) -> Template {
    Template::render("login", context! { user: UserContext::from_cookiejar(cookies), failed: failed})
}

#[post("/login", data = "<form>")]
fn login_post(form: Form<LoginForm<'_>>, cookies: &CookieJar<'_>, mut db: DbPool) -> Flash<Redirect> {
    let mut hasher = Sha256::new();
    hasher.update(form.password);
    let hash = format!("{:x}", hasher.finalize());
    let found: User = match users::table
        .filter(users::email.eq(form.email))
        .filter(users::password.eq(&hash))
        .first(&mut *db) {
            Ok(user) => user,
            Err(NotFound) => return Flash::error(Redirect::to(uri!("/auth", login(failed = Some(true)))), "Wrong cridentials"),
            Err(err) => panic!("{}", err)
        };

    cookies.add_private(Cookie::new("logged_in", "true"));
    cookies.add_private(Cookie::new("user_id", found.id.to_string()));
    cookies.add_private(Cookie::new("name", found.name));
    cookies.add_private(Cookie::new("is_admin", if found.privilege >= 1000 { "true" } else { "false" }));

    Flash::success(Redirect::to(uri!("/")), "Successfully logged in")
}

// Register

#[get("/register")]
fn register(cookies: &CookieJar<'_>) -> Template {
    Template::render("register", context!{ user: UserContext::from_cookiejar(cookies) })
}

#[post("/register", data = "<form>")]
fn register_post(form: Form<RegisterForm<'_>>, cookies: &CookieJar<'_>, mut db: DbPool) -> Flash<Redirect> {
    let mut hasher = Sha256::new();
    hasher.update(form.password);
    let hash = format!("{:x}", hasher.finalize());

    let new_user = NewUser {
        name: form.name,
        email: form.email,
        password: &hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut *db)
        .unwrap();
    let user: User = users::table.find(sql("last_insert_rowid()")).get_result(&mut *db).unwrap();

    cookies.add_private(Cookie::new("logged_in", "true"));
    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    cookies.add_private(Cookie::new("name", user.name.to_owned()));
    cookies.add_private(Cookie::new("is_admin", "false"));

    Flash::success(Redirect::to(uri!("/")), "Successfully logged in")
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("logged_in"));
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("name"));
    cookies.remove_private(Cookie::named("is_admin"));
    Flash::success(Redirect::to(uri!("/")), "Successfully logged out")
}

pub fn get_routes() -> Vec<Route> {
    routes![login, login_post, register, register_post, logout]
}
