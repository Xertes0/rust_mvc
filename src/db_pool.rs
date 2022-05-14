use std::ops::{Deref, DerefMut};

use diesel::{r2d2::{PooledConnection, ConnectionManager, Pool}, SqliteConnection};
use rocket::{request::{FromRequest, Outcome, Request}, State, http::Status};

pub struct DbPool(pub PooledConnection<ConnectionManager<SqliteConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbPool {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = req.guard::<&State<Pool<ConnectionManager<SqliteConnection>>>>().await.unwrap();
        match pool.get() {
            Ok(conn) => Outcome::Success(DbPool(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }

}

impl Deref for DbPool {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
