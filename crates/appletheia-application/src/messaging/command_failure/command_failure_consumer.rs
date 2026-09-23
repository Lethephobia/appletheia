use super::CommandFailureDelivery;
use crate::command::CommandFailureEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{
    CloudEventConsumer, CloudEventDelivery, CloudEventTypePrefix, Consumer, ConsumerError,
};

pub struct CommandFailureConsumer<C> {
    consumer: C,
    type_prefix: Option<CloudEventTypePrefix>,
}

impl<C> CommandFailureConsumer<C> {
    pub fn new(consumer: C, type_prefix: Option<CloudEventTypePrefix>) -> Self {
        Self {
            consumer,
            type_prefix,
        }
    }
}

impl<C: CloudEventConsumer> Consumer<CommandFailureEnvelope> for CommandFailureConsumer<C> {
    type Delivery = CommandFailureDelivery<C::Delivery>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
        let mut delivery = self
            .consumer
            .next()
            .await
            .map_err(|source| ConsumerError::Next(Box::new(source)))?;
        match CommandFailureEnvelope::try_from_cloud_event(
            delivery.message(),
            self.type_prefix.as_ref(),
        ) {
            Ok(message) => Ok(CommandFailureDelivery::new(delivery, message)),
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
