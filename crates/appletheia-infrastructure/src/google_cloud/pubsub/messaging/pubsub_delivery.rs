use appletheia_application::{Delivery, DeliveryError};
use google_cloud_pubsub::subscriber::handler::Handler;

pub struct PubsubDelivery<M>
where
    M: Send,
{
    handler: Option<Handler>,
    message: M,
}

impl<M> PubsubDelivery<M>
where
    M: Send,
{
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

    async fn ack(&mut self) -> Result<(), DeliveryError> {
        if let Some(handler) = self.handler.take() {
            handler.ack();
        }
        Ok(())
    }

    async fn nack(&mut self) -> Result<(), DeliveryError> {
        if let Some(handler) = self.handler.take() {
            drop(handler);
        }
        Ok(())
    }
}
