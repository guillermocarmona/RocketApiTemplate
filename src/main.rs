mod config;
mod api;
mod db;
mod utils;

#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    config::init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    rocket::build()
        .attach(AdHoc::on_ignite("API Routes", |rocket| async {
            rocket
                .mount("/api/v1", api::v1::get_routes())
        }))
}