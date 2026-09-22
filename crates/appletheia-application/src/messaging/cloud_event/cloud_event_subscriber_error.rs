use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventSubscriberError {
    #[error("CloudEvent subscribe failed")]
    Subscribe(#[source] Box<dyn StdError + Send + Sync>),
    #[error("invalid CloudEvent subscription")]
    InvalidSubscription,
    #[error("existing subscription has a different topic, filter, or ordering configuration")]
    SubscriptionConflict,
}
