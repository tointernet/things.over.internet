mod controllers;
mod openapi;
mod routes;
mod viewmodels;

use auth::jwt_manager::auth0::Auth0JwtManager;
use env::{ConfigBuilder, Configs, Empty};
use httpw::server::HttpwServerImpl;
use openapi::ApiDoc;
use std::error::Error;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = default_setup().await?;

    let jwt = Auth0JwtManager::new(&cfg.app);

    let doc = ApiDoc::openapi();

    let server = HttpwServerImpl::new(&cfg.app)
        .register(routes::things_routes())
        .register(routes::users_routes())
        .jwt_manager(jwt)
        .openapi(&doc);

    server.start().await?;

    Ok(())
}

async fn default_setup() -> Result<Configs<Empty>, Box<dyn Error>> {
    let (app_cfg, mut builder) = ConfigBuilder::new()
        .load_from_aws_secret()
        .otlp()
        .laze_load();

    logging::setup(&app_cfg)?;

    let cfg = builder.build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;

    Ok(cfg)
}
