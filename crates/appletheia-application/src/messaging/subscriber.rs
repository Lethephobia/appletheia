use super::{Consumer, ConsumerGroup, SubscriberError, Subscription};

/// Subscribes a consumer group to messages selected from a messaging backend.
#[allow(async_fn_in_trait)]
pub trait Subscriber<M>: Send + Sync {
    type Consumer: Consumer<M>;
    type Selector;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError>;
}
