use crate::controllers::things;
use actix_web::web::{self, ServiceConfig};
use httpw::server::AppConfig;

pub fn things_routes() -> AppConfig {
    |cfg: &mut ServiceConfig| {
        cfg.route("/v1/things", web::post().to(things::post));
    }
}
