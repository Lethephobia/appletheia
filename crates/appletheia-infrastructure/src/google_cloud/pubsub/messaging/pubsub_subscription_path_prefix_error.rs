use thiserror::Error;

/// Represents a validation error for a Pub/Sub subscription path prefix.
#[derive(Debug, Error)]
pub enum PubsubSubscriptionPathPrefixError {
    #[error("subscription path prefix must not be empty")]
    Empty,

    #[error("subscription path prefix must start with `projects/`")]
    InvalidFormat,
}
