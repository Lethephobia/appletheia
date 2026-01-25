use appletheia_application::{ConsumerError, Delivery};
use google_cloud_pubsub::subscriber::ReceivedMessage;

pub struct PubsubDelivery<M> {
    received_message: ReceivedMessage,
    message: M,
}

impl<M> PubsubDelivery<M> {
    pub(crate) fn new(received_message: ReceivedMessage, message: M) -> Self {
        Self {
            received_message,
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
        self.received_message
            .ack()
            .await
            .map_err(|error| ConsumerError::Ack(Box::new(error)))
    }

    async fn nack(&mut self) -> Result<(), ConsumerError> {
        self.received_message
            .nack()
            .await
            .map_err(|error| ConsumerError::Nack(Box::new(error)))
    }
}
