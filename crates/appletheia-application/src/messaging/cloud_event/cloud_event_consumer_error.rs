use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventConsumerError {
    #[error("CloudEvent next failed")]
    Next(#[source] Box<dyn StdError + Send + Sync>),
}
