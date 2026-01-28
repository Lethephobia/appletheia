use super::{ConsumerError, Delivery};

#[allow(async_fn_in_trait)]
pub trait Consumer<M>: Send {
    type Delivery: Delivery<M>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError>;
}

