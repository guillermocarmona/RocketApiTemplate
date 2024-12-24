use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::status;
use serde::Serialize;

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String
}


#[post("/login", data = "<user>")]
pub async fn login(user: Json<LoginRequest>) -> Status {
    Status::Ok
}

#[derive(Serialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
    confirm_password: String
}


#[post("/register", data = "<user>")]
pub async fn register(user: Json<RegisterRequest>) -> Status {
    Status::Ok
}
