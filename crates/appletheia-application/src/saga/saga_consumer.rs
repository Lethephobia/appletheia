use std::error::Error;

use super::{SagaDefinition, SagaDelivery};

#[allow(async_fn_in_trait)]
pub trait SagaConsumer: Send {
    type Error: Error + Send + Sync + 'static;
    type Delivery: SagaDelivery<Error = Self::Error>;
    type Saga: SagaDefinition;

    async fn next(&mut self) -> Result<Self::Delivery, Self::Error>;
}
