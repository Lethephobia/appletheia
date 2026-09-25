use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization membership fragments.
#[derive(Debug, Error)]
pub enum OrganizationMembershipFragmentWriterError {
    #[error("organization membership fragment persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
