use crate::event::EventEnvelope;
use crate::messaging::{CloudEventDelivery, Delivery, DeliveryError};

pub struct EventDelivery<D>
where
    D: CloudEventDelivery,
{
    delivery: D,
    message: EventEnvelope,
}

impl<D> EventDelivery<D>
where
    D: CloudEventDelivery,
{
    pub fn new(delivery: D, message: EventEnvelope) -> Self {
        Self { delivery, message }
    }
}

impl<D> Delivery<EventEnvelope> for EventDelivery<D>
where
    D: CloudEventDelivery,
{
    fn message(&self) -> &EventEnvelope {
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
