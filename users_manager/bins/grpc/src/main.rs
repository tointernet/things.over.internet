mod services;

use env::{ConfigBuilder, Configs, Empty};
use protos::users::users_server::UsersServer;
use services::UsersServices;
use std::{error::Error, time::Duration};
use tonic::transport::Server;
use tracing::{debug, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = default_setup().await?;

    let grpc_service = UsersServices::new();

    debug!("starting grpc server");

    Server::builder()
        .timeout(Duration::from_secs(60))
        .add_service(UsersServer::new(grpc_service))
        .serve_with_shutdown(cfg.app.app_addr().parse()?, async {
            debug!(addr = cfg.app.app_addr(), "grpc server is running");
            match tokio::spawn(tokio::signal::ctrl_c()).await {
                Err(e) => error!(
                    error = e.to_string(),
                    "server shutdown - something went wrong"
                ),
                _ => {}
            };
        })
        .await?;

    Ok(())
}

async fn default_setup() -> Result<Configs<Empty>, Box<dyn Error>> {
    let (app_cfg, mut builder) = ConfigBuilder::new()
        .load_from_aws_secret()
        .amqp()
        .aws()
        .otlp()
        .laze_load();

    logging::setup(&app_cfg)?;

    let cfg = builder.build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;

    Ok(cfg)
}
