use std::error::Error;

use thiserror::Error;

/// Error returned while writing wallet bookmark fragments.
#[derive(Debug, Error)]
pub enum WalletBookmarkFragmentWriterError {
    #[error("wallet bookmark fragment writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
