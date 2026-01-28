use super::{Consumer, ConsumerGroup, Publisher, TopicError};

#[allow(async_fn_in_trait)]
pub trait Topic<M>: Send {
    type Consumer: Consumer<M>;
    type Publisher: Publisher<M>;
    type Selector;

    fn publisher(&self) -> &Self::Publisher;

    async fn subscribe(
        &mut self,
        consumer_group: &ConsumerGroup,
        selectors: &[Self::Selector],
    ) -> Result<Self::Consumer, TopicError>;
}
