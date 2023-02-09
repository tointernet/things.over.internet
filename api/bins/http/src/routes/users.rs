use crate::controllers::users;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn users_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        cfg.service(
            web::resource("/v1/users")
                .route(web::post().to(users::post))
                .route(web::get().to(users::get)),
        );
    }
}
