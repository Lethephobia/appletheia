use thiserror::Error;

use tonic::Status;

#[derive(Debug, Error)]
pub enum PubsubOutboxPublisherError {
    #[error("failed to build pubsub message: {0}")]
    BuildMessage(#[from] serde_json::Error),

    #[error("pubsub publish failed: {0}")]
    Publish(#[source] Status),
}
