use super::{Consumer, ConsumerFactoryError, ConsumerGroup};

#[allow(async_fn_in_trait)]
pub trait ConsumerFactory<M>: Send {
    type Consumer: Consumer<M>;
    type Selector;

    async fn subscribe(
        &mut self,
        consumer_group: &ConsumerGroup,
        selectors: &[Self::Selector],
    ) -> Result<Self::Consumer, ConsumerFactoryError>;
}
