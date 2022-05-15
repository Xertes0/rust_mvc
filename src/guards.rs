use std::ops::Deref;

use rocket::{request::{FromRequest, Outcome}, Request, http::{Status, CookieJar, Cookie}};
use serde::Serialize;

const ADMIN_PRIVILEGE: i32 = 1000;

#[derive(Serialize, Clone)]
pub struct InnerUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub privilege: i32,
}

impl InnerUser {
    pub fn remove_cookies(cookies: &CookieJar) {
        cookies.remove_private(Cookie::named("user_id"));
        cookies.remove_private(Cookie::named("name"));
        cookies.remove_private(Cookie::named("email"));
        cookies.remove_private(Cookie::named("privilege"));
    }
}

#[derive(Clone)]
pub struct UserGuard(pub InnerUser);
#[derive(Clone)]
pub struct AdminUserGuard(pub InnerUser);

impl Deref for UserGuard {
    type Target = InnerUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AdminUserGuard {
    type Target = InnerUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();

        let user = || {
            let id = cookies.get_private("user_id")?.value().parse().unwrap();
            let name = cookies.get_private("name")?.value().to_owned();
            let email = cookies.get_private("email")?.value().to_owned();
            let privilege = cookies.get_private("privilege")?.value().parse().unwrap();

            Some(InnerUser { id, name, email, privilege })
        };

        if let Some(user) = user() {
            Outcome::Success(UserGuard(user))
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUserGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();

        let user = || {
            let id = cookies.get_private("user_id")?.value().parse().unwrap();
            let name = cookies.get_private("name")?.value().to_owned();
            let email = cookies.get_private("email")?.value().to_owned();
            let privilege = cookies.get_private("privilege")?.value().parse().unwrap();

            Some(InnerUser { id, name, email, privilege })
        };

        if let Some(user) = user() {
            if user.privilege >= ADMIN_PRIVILEGE {
                return Outcome::Success(AdminUserGuard(user));
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
