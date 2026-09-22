use super::{CloudEventConsumerError, CloudEventDelivery};

#[allow(async_fn_in_trait)]
pub trait CloudEventConsumer: Send {
    type Delivery: CloudEventDelivery;

    async fn next(&mut self) -> Result<Self::Delivery, CloudEventConsumerError>;
}
