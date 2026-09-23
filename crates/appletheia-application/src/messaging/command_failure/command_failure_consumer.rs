use super::CommandFailureDelivery;
use crate::command::CommandFailureEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{
    CloudEventConsumer, CloudEventDelivery, CloudEventTypePrefix, Consumer, ConsumerError,
};

pub struct CommandFailureConsumer<C>
where
    C: CloudEventConsumer,
{
    cloud_event_consumer: C,
    type_prefix: Option<CloudEventTypePrefix>,
}

impl<C> CommandFailureConsumer<C>
where
    C: CloudEventConsumer,
{
    pub fn new(cloud_event_consumer: C, type_prefix: Option<CloudEventTypePrefix>) -> Self {
        Self {
            cloud_event_consumer,
            type_prefix,
        }
    }
}

impl<C> Consumer<CommandFailureEnvelope> for CommandFailureConsumer<C>
where
    C: CloudEventConsumer,
{
    type Delivery = CommandFailureDelivery<C::Delivery>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
        let mut cloud_event_delivery = self
            .cloud_event_consumer
            .next()
            .await
            .map_err(|source| ConsumerError::Next(Box::new(source)))?;
        match CommandFailureEnvelope::try_from_cloud_event(
            cloud_event_delivery.message(),
            self.type_prefix.as_ref(),
        ) {
            Ok(message) => Ok(CommandFailureDelivery::new(cloud_event_delivery, message)),
            Err(source) => {
                cloud_event_delivery
                    .nack()
                    .await
                    .map_err(|nack_error| ConsumerError::Next(Box::new(nack_error)))?;
                Err(ConsumerError::Next(Box::new(source)))
            }
        }
    }
}
