use crate::controllers::things;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn things_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        // let mw = cfg.app_data(ext);
        // cfg.service(
        //     web::resource("/v1/things")
        //         .wrap(mw)
        //         .route(web::post().to(things::post)),
        // );

        cfg.service(web::resource("/v1/things").route(web::post().to(things::post)));
    }
}
