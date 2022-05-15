use std::collections::HashMap;

use diesel::{RunQueryDsl, query_dsl::methods::FilterDsl, ExpressionMethods};
use rocket::{Route, routes, get, response::{Redirect, Flash}, uri, http::{CookieJar, Cookie}};
use rocket_dyn_templates::{Template, context};
use serde::Serialize;

use crate::{guards::UserGuard, db_pool::DbPool, schema, models};

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

pub fn get_routes() -> Vec<Route> {
    routes![index, list, to_cart, cart, delete_from_cart]
}
