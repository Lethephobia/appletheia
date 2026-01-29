use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PublisherError {
    #[error("publish error")]
    Publish(#[source] Box<dyn Error + Send + Sync>),
}
