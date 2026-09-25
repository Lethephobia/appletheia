use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization invitation fragments.
#[derive(Debug, Error)]
pub enum OrganizationInvitationFragmentWriterError {
    #[error("organization invitation fragment persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
