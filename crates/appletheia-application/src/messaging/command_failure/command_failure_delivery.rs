use crate::command::CommandFailureEnvelope;
use crate::messaging::{CloudEventDelivery, Delivery, DeliveryError};

pub struct CommandFailureDelivery<D> {
    delivery: D,
    message: CommandFailureEnvelope,
}

impl<D> CommandFailureDelivery<D> {
    pub fn new(delivery: D, message: CommandFailureEnvelope) -> Self {
        Self { delivery, message }
    }
}

impl<D: CloudEventDelivery> Delivery<CommandFailureEnvelope> for CommandFailureDelivery<D> {
    fn message(&self) -> &CommandFailureEnvelope {
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
