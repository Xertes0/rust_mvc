use rocket::{get, Route, routes, response::Redirect, uri};

//#[get("/")]
//fn index(user: Option<UserGuard>) -> Template {
//    // TODO don't clone
//    Template::render("index", context!{ user: user.map(|x| (*x).clone()) })
//}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/products/list"))
}

pub fn get_routes() -> Vec<Route> {
    routes![index]
}
