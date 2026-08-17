use std::error::Error;

use thiserror::Error;

/// Error returned while writing currency fragments.
#[derive(Debug, Error)]
pub enum CurrencyFragmentWriterError {
    #[error("currency fragment writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
