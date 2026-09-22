use super::{CloudEvent, CloudEventDeliveryError};

#[allow(async_fn_in_trait)]
pub trait CloudEventDelivery: Send {
    fn message(&self) -> &CloudEvent;

    async fn ack(&mut self) -> Result<(), CloudEventDeliveryError>;

    async fn nack(&mut self) -> Result<(), CloudEventDeliveryError>;
}
