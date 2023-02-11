use actix_web::{HttpRequest, HttpResponse, Responder};
use httpw::extractors::JwtAuthenticateExtractor;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {}

pub async fn post(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("post::users")
}

pub async fn get(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("get::users")
}
