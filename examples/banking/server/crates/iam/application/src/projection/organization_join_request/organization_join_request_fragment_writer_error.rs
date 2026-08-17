use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization join request fragments.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestFragmentWriterError {
    #[error("organization join request fragment persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
