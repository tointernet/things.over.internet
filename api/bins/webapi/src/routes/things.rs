use crate::controllers::things;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn things_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/v1/things")
                .route("/", web::post().to(things::post))
                .route("/", web::get().to(things::list))
                .route("/{user_id}", web::get().to(things::get))
                .route("/{user_id}", web::delete().to(things::delete)),
        );
    }
}
