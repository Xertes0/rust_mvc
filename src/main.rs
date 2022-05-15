use dotenv::dotenv;
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};
use rocket::launch;
use rocket_dyn_templates::Template;
use std::env;

pub mod schema;
pub mod db_pool;
pub mod guards;
mod models;
mod routes;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let connection = ConnectionManager::<SqliteConnection>::new(&env::var("DATABASE_URL").expect("No DATABASE_URL env var"));
    let pool = Pool::new(connection).expect("db pool");

    rocket::build()
        .manage(pool)
        .attach(Template::fairing())
        .mount("/", routes::index::get_routes())
        .mount("/auth", routes::auth::get_routes())
        .mount("/users", routes::users::get_routes())
}
