use amqp::topology::ConsumerHandler;
use async_trait::async_trait;
use errors::amqp::AmqpError;
use opentelemetry::Context;
use std::sync::Arc;

pub struct ConsumeUsers {}

impl ConsumeUsers {
    pub fn new() -> Arc<ConsumeUsers> {
        Arc::new(ConsumeUsers {})
    }
}

#[async_trait]
impl ConsumerHandler for ConsumeUsers {
    async fn exec(&self, ctx: &Context, data: &[u8]) -> Result<(), AmqpError> {
        Ok(())
    }
}
