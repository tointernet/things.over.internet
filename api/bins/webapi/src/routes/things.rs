use crate::controllers::things;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn things_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/v1/things")
                .service(things::post)
                .service(things::list)
                .service(things::get)
                .service(things::delete),
        );
    }
}
