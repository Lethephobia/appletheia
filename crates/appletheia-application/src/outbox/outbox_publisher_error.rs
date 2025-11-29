use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxPublisherError {
    #[error("outbox publish error: {0}")]
    Publish(#[source] Box<dyn Error + Send + Sync + 'static>),
}
