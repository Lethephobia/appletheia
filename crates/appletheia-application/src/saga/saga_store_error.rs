use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SagaStoreError {
    #[error("not in transaction")]
    NotInTransaction,
    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
