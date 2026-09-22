use super::DeliveryError;

#[allow(async_fn_in_trait)]
pub trait Delivery<M>: Send {
    fn message(&self) -> &M;

    async fn ack(&mut self) -> Result<(), DeliveryError>;

    async fn nack(&mut self) -> Result<(), DeliveryError>;
}
