use std::error::Error;

use thiserror::Error;

/// Error returned while writing a shared public user fragment.
#[derive(Debug, Error)]
pub enum UserFragmentWriterError {
    #[error("public user fragment persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
