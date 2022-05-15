use diesel::{RunQueryDsl, dsl::sql, QueryDsl, ExpressionMethods, result::Error::NotFound};
use rocket::{form::Form, {FromForm, http::{CookieJar, Cookie}, response::{Flash, Redirect}, uri, get, post}, Route, routes};
use rocket_dyn_templates::{Template, context};
use sha2::{Sha256, Digest};

use crate::{db_pool::DbPool, models::{User, NewUser}, schema::users, guards::{UserGuard, InnerUser}};
//use super::UserContext;

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
fn login(user: Option<UserGuard>, failed: Option<bool>) -> Template {
    Template::render("login", context! { user: user.map(|x| (*x).clone()), failed: failed})
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

    cookies.add_private(Cookie::new("user_id", found.id.to_string()));
    cookies.add_private(Cookie::new("name", found.name));
    cookies.add_private(Cookie::new("email", found.email));
    cookies.add_private(Cookie::new("privilege", found.privilege.to_string()));

    Flash::success(Redirect::to(uri!("/")), "Successfully logged in")
}

// Register

#[get("/register?<failed>")]
fn register(user: Option<UserGuard>, failed: Option<bool>) -> Template {
    Template::render("register", context!{ user: user.map(|x| Some((*x).clone())), failed: failed })
}

#[post("/register", data = "<form>")]
fn register_post(form: Form<RegisterForm<'_>>, cookies: &CookieJar<'_>, mut db: DbPool) -> Flash<Redirect> {
    let user: Result<User, diesel::result::Error> = users::table
        .filter(users::email.eq(form.email))
        .first(&mut *db);
    if let Ok(_) = user {
        return Flash::error(Redirect::to(uri!("/auth", register(failed = Some(true)))), "User already exists")
    }

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

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    cookies.add_private(Cookie::new("name", user.name.to_owned()));
    cookies.add_private(Cookie::new("email", user.email.to_owned()));
    cookies.add_private(Cookie::new("privilege", user.privilege.to_string()));

    Flash::success(Redirect::to(uri!("/")), "Successfully logged in")
}

#[get("/logout")]
fn logout(cookies: &CookieJar) -> Flash<Redirect> {
    InnerUser::remove_cookies(cookies);
    Flash::success(Redirect::to(uri!("/")), "Successfully logged out")
}

pub fn get_routes() -> Vec<Route> {
    routes![login, login_post, register, register_post, logout]
}
