use super::{ConsumerError, Delivery};

#[allow(async_fn_in_trait)]
pub trait Consumer<M>: Send {
    type Delivery: Delivery<M>;

    /// Waits for the next delivery. Must be cancellation safe: dropping this future
    /// must not lose a delivery or prevent a subsequent receive. Workers cancel it
    /// when stopping; processing and acknowledgement of returned deliveries finish normally.
    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError>;
}
