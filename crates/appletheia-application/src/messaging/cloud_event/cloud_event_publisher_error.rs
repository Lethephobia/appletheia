use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventPublisherError {
    #[error("CloudEvent publish failed")]
    Publish(#[source] Box<dyn StdError + Send + Sync>),
}
