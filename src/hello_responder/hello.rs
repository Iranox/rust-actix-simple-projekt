use actix_web::{get, post, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    pub name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/test")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(MyObj {
        name: String::from("user"),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
