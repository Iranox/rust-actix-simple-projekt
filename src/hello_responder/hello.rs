use actix_web::{get, post, HttpResponse, Responder};
use juniper::FieldResult;
use juniper::GraphQLObject;

use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
struct Greetings {
    pub name: String,
}

pub struct GreetingQuery;

#[juniper::graphql_object]
impl GreetingQuery {
    fn greetingName(name: String) -> FieldResult<Greetings> {
        Ok(Greetings { name })
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/test")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(Greetings {
        name: String::from("user"),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
