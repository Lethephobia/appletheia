use std::error::Error;

use thiserror::Error;

/// Error returned while writing account transaction fragment read models.
#[derive(Debug, Error)]
pub enum AccountTransactionFragmentWriterError {
    #[error("account transaction fragment writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
