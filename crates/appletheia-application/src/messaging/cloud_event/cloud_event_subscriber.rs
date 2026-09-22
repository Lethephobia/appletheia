use crate::messaging::{ConsumerGroup, Subscription};

use super::{CloudEventConsumer, CloudEventSelector, CloudEventSubscriberError};

#[allow(async_fn_in_trait)]
pub trait CloudEventSubscriber: Send + Sync {
    type Consumer: CloudEventConsumer;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, CloudEventSelector>,
    ) -> Result<Self::Consumer, CloudEventSubscriberError>;
}
