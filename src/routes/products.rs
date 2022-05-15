use std::{collections::HashMap, path::Path};

use diesel::{RunQueryDsl, query_dsl::methods::FilterDsl, ExpressionMethods};
use rocket::{Route, routes, get, response::{Redirect, Flash}, uri, http::{CookieJar, Cookie}, FromForm, post, form::Form, fs::TempFile};
use rocket_dyn_templates::{Template, context};
use serde::Serialize;

use crate::{guards::{UserGuard, AdminUserGuard}, db_pool::DbPool, schema, models};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/products", list()))
}

#[get("/list")]
fn list(user: Option<UserGuard>, mut db: DbPool) -> Template {
    let prods: Vec<models::Product> = schema::products::table
        .get_results(&mut *db).unwrap();
    Template::render("products_list", context! { user: user.map(|x| (*x).clone()), products: prods })
}

#[get("/to_cart/<id>")]
fn to_cart(id: i32, cookies: &CookieJar) -> Redirect {
    match cookies.get("cart") {
        Some(cookie) => {
            cookies.add(Cookie::new("cart", format!("{}{},", cookie.value(), id)));
        },
        None => {
            cookies.add(Cookie::new("cart", format!("{},", id)));
        },
    }

    Redirect::to(uri!("/products", list()))
}

#[derive(Serialize, Clone)]
struct CartEntry {
    count: i32,
    item: models::Product,
}

#[get("/cart")]
fn cart(user: Option<UserGuard>, cookies: &CookieJar, mut db: DbPool) -> Template {
    let entries = match cookies.get("cart") {
        Some(cookie) => {
            let mut map = HashMap::<i32, i32>::new();
            for val in cookie.value().split(',').rev().skip(1) {
                let val = &val.parse::<i32>().unwrap();
                map.insert(*val, if map.contains_key(&val) { map.get(val).unwrap() + 1 } else { 1 });
            }

            let ids: Vec<&i32> = map.keys().collect();
            let items: Vec<models::Product> = schema::products::table
                .filter(schema::products::id.eq_any(&ids))
                .get_results(&mut *db).unwrap();

            let mut entries = Vec::new();
            for (id, count) in map.iter() {
                entries.push(CartEntry {
                    count: *count,
                    item: items.iter().find(|x| x.id == *id).unwrap().clone()
                })
            }

            //Some(entries)
            entries
        },
        None => {
            //None
            Vec::new()
        }
    };

    Template::render("products_cart", context! {user: user.map(|x| (*x).clone()), entries: entries})
}

#[get("/delete_from_cart/<id>")]
fn delete_from_cart(id: i32, cookies: &CookieJar) -> Flash<Redirect> {
    match cookies.get("cart") {
        Some(cookie) => {
            let cart: Vec<&str> = cookie.value().split(",").collect();
            let mut cart: Vec<&&str> = cart.iter().rev().skip(1).collect();
            let mut removed = false;
            cart.retain(|x| {
                if !removed {
                    if x.parse::<i32>().unwrap() == id {
                        removed = true;
                        return false;
                    }
                }
                true
            });

            let mut new_cookie = String::new();
            for var in cart {
                new_cookie.push_str(&format!("{},", var));
            }
            if new_cookie.is_empty() {
                cookies.remove(Cookie::named("cart"));
            } else {
                cookies.add(Cookie::new("cart", new_cookie));
            }

            Flash::success(Redirect::to(uri!("/products", cart())), "Successfully removed one item from cart")
        },
        None => {
            Flash::error(Redirect::to(uri!("/products", cart())), "Cart is empty")
        }
    }
}

#[get("/delete/<id>")]
fn delete(id: i32, _user: AdminUserGuard, mut db: DbPool) -> Flash<Redirect> {
    match diesel::delete(schema::products::table.filter(schema::products::id.eq(id)))
        .execute(&mut *db) {
            Ok(_) => {
                Flash::success(Redirect::to(uri!("/products", list())), "Successfully deleted one product")
            },
            Err(_) => {
                Flash::error(Redirect::to(uri!("/products", list())), "Could not delete the product")
            }
        }
}

#[get("/edit/<id>")]
fn edit(id: i32, user: AdminUserGuard, mut db: DbPool) -> Template {
    let edit_prod: models::Product = match schema::products::table
        .filter(schema::products::id.eq(id))
        .first(&mut *db) {
            Ok(prod) => prod,
            Err(err) => panic!("{err}")
        };

    Template::render("products_edit", context! {user: &*user, edit_prod: edit_prod})
}

#[derive(FromForm)]
struct NewEditForm<'a> {
    name: &'a str,
    image_url: &'a str,
    image_file: TempFile<'a>,
    price: f32,
    description: &'a str,
}

#[post("/edit/<id>", data="<form>")]
async fn edit_post(id: i32, mut form: Form<NewEditForm<'_>>, _user: AdminUserGuard, mut db: DbPool) -> Flash<Redirect> {
    let path_str = format!("static/product_images/{}", id);
    let image = match form.image_file.len() {
        0 => form.image_url.to_owned(),
        _ => {
            let path = Path::new(&path_str);
            form.image_file.copy_to(path).await.unwrap(); // can use persist_to if target path is
                                                          // on the mount point
            format!("/{}", path_str)
        }
    };

    match diesel::update(schema::products::table.filter(schema::products::id.eq(id)))
        .set((
                schema::products::name.eq(form.name),
                schema::products::image.eq(image),
                schema::products::price.eq((form.price * 100.0) as i32),
                schema::products::description.eq(form.description)
            ))
        .execute(&mut *db) {
            Ok(_) => Flash::success(Redirect::to(uri!("/products", list())), "Successfully updated one product"),
            Err(_) => Flash::error(Redirect::to(uri!("/products", list())), "Could not update the product")
        }
}

#[get("/new")]
fn new(user: AdminUserGuard) -> Template {
    Template::render("products_new", context! {user: &*user})
}

#[post("/new", data="<form>")]
async fn new_post(mut form: Form<NewEditForm<'_>>, mut db: DbPool) -> Flash<Redirect> {
    let image = match form.image_file.len() {
        0 => form.image_url.to_owned(),
        _ => {
            let path_str = format!("static/product_images/{}", form.image_file.name().unwrap());
            let path = Path::new(&path_str);
            form.image_file.copy_to(path).await.unwrap(); // can use persist_to if target path is
                                                          // on the mount point
            format!("/{}", path_str)
        }
    };

    match diesel::insert_into(schema::products::table)
        .values((
                schema::products::name.eq(form.name),
                schema::products::image.eq(image),
                schema::products::price.eq((form.price * 100.0) as i32),
                schema::products::description.eq(form.description)
            ))
        .execute(&mut *db) {
            Ok(_) => Flash::success(Redirect::to(uri!("/products", list())), "Successfully inserted one product"),
            Err(_) => Flash::error(Redirect::to(uri!("/products", list())), "Could not insert product")
        }
}

pub fn get_routes() -> Vec<Route> {
    routes![index, list, to_cart, cart, delete_from_cart, delete, edit, edit_post, new, new_post]
}
