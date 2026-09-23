use crate::command::CommandEnvelope;
use crate::messaging::{CloudEventDelivery, Delivery, DeliveryError};

pub struct CommandDelivery<D>
where
    D: CloudEventDelivery,
{
    delivery: D,
    message: CommandEnvelope,
}

impl<D> CommandDelivery<D>
where
    D: CloudEventDelivery,
{
    pub fn new(delivery: D, message: CommandEnvelope) -> Self {
        Self { delivery, message }
    }
}

impl<D> Delivery<CommandEnvelope> for CommandDelivery<D>
where
    D: CloudEventDelivery,
{
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
