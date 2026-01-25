use super::ConsumerError;

#[allow(async_fn_in_trait)]
pub trait Delivery<M>: Send {
    fn message(&self) -> &M;

    async fn ack(&mut self) -> Result<(), ConsumerError>;

    async fn nack(&mut self) -> Result<(), ConsumerError>;
}

