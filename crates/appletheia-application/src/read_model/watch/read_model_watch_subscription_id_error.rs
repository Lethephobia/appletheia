use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid watch-subscription identifier.
#[derive(Debug, Error)]
pub enum ReadModelWatchSubscriptionIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
