use rocket::Route;
use crate::api::v1::user_handlers::{login, register};

pub fn get_routes() -> Vec<Route> {
    routes![
        login,
        signin,
    ]
}
