use std::error::Error;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ProjectionCheckpointStoreError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
