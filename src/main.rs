use dotenv::dotenv;
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};
use rocket::{launch, data::{Limits, ToByteUnit}};
use rocket_dyn_templates::{Template, tera::{self, Value, Filter, to_value}};
use std::{env, collections::HashMap};

pub mod schema;
pub mod db_pool;
pub mod guards;
mod models;
mod routes;

struct Localise;
impl Filter for Localise {
    fn filter(&self, value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
        match value {
            Value::Number(num) => {
                let loc = locale::Numeric::new(",", " ");
                Ok(to_value(loc.format_float(num, 2)).unwrap())
            },
            _ => panic!("Can only use numbers")
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let connection = ConnectionManager::<SqliteConnection>::new(&env::var("DATABASE_URL").expect("No DATABASE_URL env var"));
    let pool = Pool::new(connection).expect("db pool");

    let limits = Limits::default()
        .limit("file", 5.megabytes());

    let figment = rocket::Config::figment()
        .merge(("limits", limits));

    rocket::custom(figment)
        .manage(pool)
        //.attach(Template::fairing())
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("localise", Localise)
        }))
        .mount("/", routes::index::get_routes())
        .mount("/auth", routes::auth::get_routes())
        .mount("/users", routes::users::get_routes())
        .mount("/products", routes::products::get_routes())
}
