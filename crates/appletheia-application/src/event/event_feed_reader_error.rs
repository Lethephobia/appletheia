use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventFeedReaderError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn StdError + Send + Sync>),

    #[error("invalid subscription")]
    InvalidSubscription,
}
