use std::error::Error;

use thiserror::Error;

/// Error returned while writing user identity fragments.
#[derive(Debug, Error)]
pub enum UserIdentityFragmentWriterError {
    #[error("user identity fragment persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
