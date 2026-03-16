use std::error::Error;

use thiserror::Error;

/// Represents a failure while creating or configuring a subscription.
#[derive(Debug, Error)]
pub enum SubscriberError {
    #[error("subscribe error")]
    Subscribe(#[source] Box<dyn Error + Send + Sync>),

    #[error("invalid subscription")]
    InvalidSubscription,
}
