use std::error::Error;

use thiserror::Error;

/// Error returned while loading account list read models.
#[derive(Debug, Error)]
pub enum OwnedAccountListStoreError {
    #[error("account list store persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
