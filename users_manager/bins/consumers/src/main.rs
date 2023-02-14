mod consumers;

use amqp::{
    client::{Amqp, AmqpImpl},
    dispatcher::Dispatcher,
    topology::{AmqpTopology, ExchangeDefinition, QueueDefinition},
};
use consumers::ConsumeUsers;
use env::{ConfigBuilder, Configs, Empty};
use health_readiness::HealthReadinessServer;
use std::{error::Error, sync::Arc};
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = default_setup().await?;

    let (amqp, mut dispatcher, queue) = amqp_setup(&cfg).await?;

    declare_consumer(&mut dispatcher, queue.clone());

    let health_readiness =
        HealthReadinessServer::new(&cfg.health_readiness).rabbitmq(amqp.connection());

    match tokio::join!(health_readiness.run(), dispatcher.consume_blocking()) {
        (Err(e), _) => {
            error!(error = e.to_string(), "error");
            panic!("{:?}", e)
        }
        (Ok(_), errors) => {
            for err in errors {
                if err.is_err() {
                    error!("error");
                    panic!("{:?}", err)
                }
            }
        }
    }

    Ok(())
}

async fn default_setup() -> Result<Configs<Empty>, Box<dyn Error>> {
    let (app_cfg, mut builder) = ConfigBuilder::new()
        .load_from_aws_secret()
        .amqp()
        .aws()
        .dynamodb()
        .otlp()
        .laze_load();

    logging::setup(&app_cfg)?;

    let cfg = builder.build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;
    metrics::otlp::setup(&cfg)?;

    Ok(cfg)
}

async fn amqp_setup(
    cfg: &Configs<Empty>,
) -> Result<(Arc<dyn Amqp + Send + Sync>, Dispatcher, QueueDefinition), Box<dyn Error>> {
    let amqp = AmqpImpl::new(cfg).await?;
    let dispatches = Dispatcher::new(amqp.clone());

    let queue = QueueDefinition::name("SOME_QUEUE")
        .with_retry(18000, 3)
        .with_dlq()
        .binding("SOME_EXCHANGE", "SOME_KEY");

    AmqpTopology::new(amqp.clone())
        .exchange(ExchangeDefinition::name("SOME_EXCHANGE").direct())
        .queue(queue.clone())
        .install()
        .await?;

    Ok((amqp, dispatches, queue))
}

fn declare_consumer(dispatcher: &mut Dispatcher, queue: QueueDefinition) {
    let handler = ConsumeUsers::new();

    dispatcher
        .declare(queue.clone(), "message_type".to_owned(), handler.clone())
        .expect("unexpected error while registering dispatch");
}
