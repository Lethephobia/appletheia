pub mod consumer_builder;
pub mod consumer_builder_error;
pub mod consumer_error;
pub mod delivery;

pub use consumer_builder::ConsumerBuilder;
pub use consumer_builder_error::ConsumerBuilderError;
pub use consumer_error::ConsumerError;
pub use delivery::Delivery;

#[allow(async_fn_in_trait)]
pub trait Consumer<M>: Send {
    type Delivery: Delivery<M>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError>;
}
