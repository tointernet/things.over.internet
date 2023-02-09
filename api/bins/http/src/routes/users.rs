use std::sync::Arc;

use crate::controllers::users;
use actix_web::{
    dev::Service,
    web::{self, ServiceConfig},
};
use auth::AuthMiddleware;
use httpw::server::RouteConfig;

pub fn users_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        // let mw = cfg.app_data(ext);
        cfg.service(
            web::resource("/v1/things")
                .wrap_fn(|req, srv| {
                    let mid = req
                        .app_data::<Arc<dyn AuthMiddleware + Send + Sync>>()
                        .map(|data| data.clone())
                        .unwrap();

                    // mid.authenticate(ctx, token);

                    srv.call(req)
                })
                .route(web::post().to(users::post)),
        );

        // cfg.route("/v1/users", web::post().to(users::post));
    }
}
