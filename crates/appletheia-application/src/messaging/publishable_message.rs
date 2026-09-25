use super::OrderingKey;

/// Defines the ordering scope used when publishing a message.
pub trait PublishableMessage: Send + Sync {
    fn ordering_key(&self) -> OrderingKey;
}
