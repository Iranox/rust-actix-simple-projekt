#[path = "model/mod.rs"]
mod model;

use actix_web::{get, HttpResponse, Responder};

#[get("/user")]
async fn user() -> impl Responder {
    let user = model::user::User {
        username: String::from("user"),
        email: String::from("test@example.com"),
    };
    HttpResponse::Ok().json(user)
}
