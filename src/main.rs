use dotenv::dotenv;
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};
use rocket::launch;
use rocket_dyn_templates::{Template, tera::{Value, Filter, to_value}};
use std::env;

pub mod schema;
pub mod db_pool;
pub mod guards;
mod models;
mod routes;

struct NumberFilter;
impl Filter for NumberFilter {
    fn filter(&self, value: &Value, _: &std::collections::HashMap<String, Value>) -> rocket_dyn_templates::tera::Result<Value> {
        match value {
            Value::Number(num) => {
                let loc = locale::Numeric::new(",", " ");
                Ok(to_value(loc.format_float(num, 2)).unwrap())
            },
            _ => { panic!("Can only use numbers") }
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let connection = ConnectionManager::<SqliteConnection>::new(&env::var("DATABASE_URL").expect("No DATABASE_URL env var"));
    let pool = Pool::new(connection).expect("db pool");

    rocket::build()
        .manage(pool)
        //.attach(Template::fairing())
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("localise", NumberFilter)
        }))
        .mount("/", routes::index::get_routes())
        .mount("/auth", routes::auth::get_routes())
        .mount("/users", routes::users::get_routes())
        .mount("/products", routes::products::get_routes())
}
