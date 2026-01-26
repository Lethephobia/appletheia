pub mod consumer_error;
pub mod consumer_factory;
pub mod consumer_factory_error;
pub mod consumer_group;
pub mod consumer_group_error;
pub mod delivery;

pub use consumer_error::ConsumerError;
pub use consumer_factory::ConsumerFactory;
pub use consumer_factory_error::ConsumerFactoryError;
pub use consumer_group::ConsumerGroup;
pub use consumer_group_error::ConsumerGroupError;
pub use delivery::Delivery;

#[allow(async_fn_in_trait)]
pub trait Consumer<M>: Send {
    type Delivery: Delivery<M>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError>;
}
