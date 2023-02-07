use crate::controllers::users;
use actix_web::web::{self, ServiceConfig};
use httpw::server::AppConfig;

pub fn users_routes() -> AppConfig {
    |cfg: &mut ServiceConfig| {
        cfg.route("/v1/users", web::post().to(users::post));
    }
}
