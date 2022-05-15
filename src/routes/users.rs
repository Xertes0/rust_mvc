use diesel::{RunQueryDsl, ExpressionMethods, query_dsl::methods::FilterDsl};
use rocket::{Route, routes, get, response::{Redirect, Flash}, uri, post, form::Form, FromForm};
use rocket_dyn_templates::{Template, context};

use crate::{guards::AdminUserGuard, db_pool::DbPool, schema::users, models};

#[get("/list")]
fn list(user: AdminUserGuard, mut db: DbPool) -> Template {
    let users: Vec<models::User> = users::table
        .get_results(&mut *db).unwrap();

    Template::render("users_list", context! {user: &*user, users: &users})
}

#[get("/delete/<id>")]
fn delete(id: i32, _user: AdminUserGuard, mut db: DbPool) -> Flash<Redirect> {
    match diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(&mut *db) {
            Ok(_) => Flash::success(Redirect::to(uri!("/users", list())), "Deleted one user"),
            Err(_) => Flash::error(Redirect::to(uri!("/users", list())), "Could not delete the user")
        }
}

#[get("/edit/<id>")]
fn edit(id: i32, user: AdminUserGuard, mut db: DbPool) -> Template {
    let edit_user: models::User = users::table
        .filter(users::id.eq(id))
        .first(&mut *db).unwrap();
    Template::render("users_edit", context! {user: &*user, edit_user: &edit_user})
}

#[derive(FromForm)]
struct EditForm<'a> {
    name: &'a str,
    email: &'a str,
    privilege: i32,
}

#[post("/edit/<id>", data = "<form>")]
fn edit_post(id: i32, form: Form<EditForm<'_>>, _user: AdminUserGuard, mut db: DbPool) -> Redirect {
    diesel::update(users::table.filter(users::id.eq(id)))
        .set((users::name.eq(form.name), users::email.eq(form.email), users::privilege.eq(form.privilege)))
        .execute(&mut *db).unwrap();

    Redirect::to(uri!("/users", list()))
}

pub fn get_routes() -> Vec<Route> {
    routes![list, delete, edit, edit_post]
}
