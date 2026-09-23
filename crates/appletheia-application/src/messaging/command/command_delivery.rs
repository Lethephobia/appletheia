use crate::command::CommandEnvelope;
use crate::messaging::{CloudEventDelivery, Delivery, DeliveryError};

pub struct CommandDelivery<D> {
    delivery: D,
    message: CommandEnvelope,
}

impl<D> CommandDelivery<D> {
    pub fn new(delivery: D, message: CommandEnvelope) -> Self {
        Self { delivery, message }
    }
}

impl<D: CloudEventDelivery> Delivery<CommandEnvelope> for CommandDelivery<D> {
    fn message(&self) -> &CommandEnvelope {
        &self.message
    }

    async fn ack(&mut self) -> Result<(), DeliveryError> {
        self.delivery
            .ack()
            .await
            .map_err(|source| DeliveryError::Ack(Box::new(source)))
    }

    async fn nack(&mut self) -> Result<(), DeliveryError> {
        self.delivery
            .nack()
            .await
            .map_err(|source| DeliveryError::Nack(Box::new(source)))
    }
}
