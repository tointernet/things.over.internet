use actix_web::{delete, get, post, web::Json, HttpRequest, HttpResponse, Responder};
use httpw::extractors::JwtAuthenticateExtractor;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
struct Info {}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Thing {}

/// Create a new thing.
///
/// Post a new `Thing` in request body as json. Api will return
/// accepted `Thing` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
#[utoipa::path(
    request_body = Thing,
    responses(
        (status = 202, description = "Thing requested successfully", body = Thing),
        (status = 409, description = "Thing conflict", body = ErrorResponse)
    )
)]
#[post("/")]
pub async fn post(_thing: Json<Thing>, _auth: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("post::things")
}

#[get("/")]
pub async fn list(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("list::things")
}

#[get("/{id}")]
pub async fn get(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("get::things")
}

#[delete("/{id}")]
pub async fn delete(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("delete::things")
}
