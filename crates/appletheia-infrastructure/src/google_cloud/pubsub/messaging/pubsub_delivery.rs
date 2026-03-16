use appletheia_application::{ConsumerError, Delivery};
use google_cloud_pubsub::subscriber::handler::Handler;

pub struct PubsubDelivery<M> {
    handler: Option<Handler>,
    message: M,
}

impl<M> PubsubDelivery<M> {
    pub(crate) fn new(handler: Handler, message: M) -> Self {
        Self {
            handler: Some(handler),
            message,
        }
    }
}

impl<M> Delivery<M> for PubsubDelivery<M>
where
    M: Send,
{
    fn message(&self) -> &M {
        &self.message
    }

    async fn ack(&mut self) -> Result<(), ConsumerError> {
        if let Some(handler) = self.handler.take() {
            handler.ack();
        }
        Ok(())
    }

    async fn nack(&mut self) -> Result<(), ConsumerError> {
        if let Some(handler) = self.handler.take() {
            drop(handler);
        }
        Ok(())
    }
}
