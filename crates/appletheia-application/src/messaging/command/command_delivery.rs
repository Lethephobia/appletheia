use crate::command::CommandEnvelope;
use crate::messaging::{CloudEventDelivery, Delivery, DeliveryError};

pub struct CommandDelivery<D>
where
    D: CloudEventDelivery,
{
    cloud_event_delivery: D,
    message: CommandEnvelope,
}

impl<D> CommandDelivery<D>
where
    D: CloudEventDelivery,
{
    pub fn new(cloud_event_delivery: D, message: CommandEnvelope) -> Self {
        Self {
            cloud_event_delivery,
            message,
        }
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
        self.cloud_event_delivery
            .ack()
            .await
            .map_err(|source| DeliveryError::Ack(Box::new(source)))
    }

    async fn nack(&mut self) -> Result<(), DeliveryError> {
        self.cloud_event_delivery
            .nack()
            .await
            .map_err(|source| DeliveryError::Nack(Box::new(source)))
    }
}
