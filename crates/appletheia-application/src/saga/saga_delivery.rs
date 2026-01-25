use std::error::Error;

use crate::event::EventEnvelope;

#[allow(async_fn_in_trait)]
pub trait SagaDelivery: Send {
    type Error: Error + Send + Sync + 'static;

    fn event(&self) -> &EventEnvelope;

    async fn ack(&mut self) -> Result<(), Self::Error>;

    async fn nack(&mut self) -> Result<(), Self::Error>;
}

