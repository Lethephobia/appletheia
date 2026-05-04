use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectionCheckpointStoreError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn StdError + Send + Sync>),
}
