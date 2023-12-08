use actix_web::{get, http::StatusCode, post, HttpResponse, Responder};
use log::warn;

#[get("/")]
pub async fn hello() -> impl Responder {
    warn!("access hello");
    HttpResponse::Ok().body(StatusCode::OK.as_str())
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().status(StatusCode::OK).body("Hey there!")
}
