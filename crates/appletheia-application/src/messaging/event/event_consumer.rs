use super::EventDelivery;
use crate::event::EventEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{
    CloudEventConsumer, CloudEventDelivery, CloudEventTypePrefix, Consumer, ConsumerError,
};

pub struct EventConsumer<C> {
    consumer: C,
    type_prefix: Option<CloudEventTypePrefix>,
}

impl<C> EventConsumer<C> {
    pub fn new(consumer: C, type_prefix: Option<CloudEventTypePrefix>) -> Self {
        Self {
            consumer,
            type_prefix,
        }
    }
}

impl<C: CloudEventConsumer> Consumer<EventEnvelope> for EventConsumer<C> {
    type Delivery = EventDelivery<C::Delivery>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
        let mut delivery = self
            .consumer
            .next()
            .await
            .map_err(|source| ConsumerError::Next(Box::new(source)))?;
        match EventEnvelope::try_from_cloud_event(delivery.message(), self.type_prefix.as_ref()) {
            Ok(message) => Ok(EventDelivery::new(delivery, message)),
            Err(source) => {
                delivery
                    .nack()
                    .await
                    .map_err(|nack_error| ConsumerError::Next(Box::new(nack_error)))?;
                Err(ConsumerError::Next(Box::new(source)))
            }
        }
    }
}
