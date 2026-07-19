use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization invitation lists.
#[derive(Debug, Error)]
pub enum OrganizationInvitationListWriterError {
    #[error("organization invitation list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
