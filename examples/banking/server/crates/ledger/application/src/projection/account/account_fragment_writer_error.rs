use std::error::Error;

use thiserror::Error;

/// Error returned while writing account fragments.
#[derive(Debug, Error)]
pub enum AccountFragmentWriterError {
    #[error("account fragment writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
