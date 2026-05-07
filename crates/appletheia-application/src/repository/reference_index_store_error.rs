use std::error::Error;

use thiserror::Error;

/// Errors returned by `ReferenceIndexStore`.
#[derive(Debug, Error)]
pub enum ReferenceIndexStoreError {
    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}
