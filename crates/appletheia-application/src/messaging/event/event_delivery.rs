use crate::event::EventEnvelope;
use crate::messaging::{CloudEventDelivery, Delivery, DeliveryError};

pub struct EventDelivery<D>
where
    D: CloudEventDelivery,
{
    cloud_event_delivery: D,
    message: EventEnvelope,
}

impl<D> EventDelivery<D>
where
    D: CloudEventDelivery,
{
    pub fn new(cloud_event_delivery: D, message: EventEnvelope) -> Self {
        Self {
            cloud_event_delivery,
            message,
        }
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
