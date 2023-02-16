use crate::controllers::users;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn users_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/v1/users")
                .route("/", web::post().to(users::post))
                .route("/", web::get().to(users::list))
                .route("/{user_id}", web::get().to(users::get))
                .route("/{user_id}", web::patch().to(users::patch))
                .route("/{user_id}", web::delete().to(users::delete)),
        );
    }
}
