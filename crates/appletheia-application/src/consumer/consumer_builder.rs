use super::{Consumer, ConsumerBuilderError};

#[allow(async_fn_in_trait)]
pub trait ConsumerBuilder<M>: Send {
    type Consumer: Consumer<M>;
    type Selector;

    async fn subscribe(
        &mut self,
        consumer_group: &str,
        selectors: &[Self::Selector],
    ) -> Result<Self::Consumer, ConsumerBuilderError>;
}

