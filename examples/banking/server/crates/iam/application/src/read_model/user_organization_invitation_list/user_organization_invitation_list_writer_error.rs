use std::error::Error;

use thiserror::Error;

/// Error returned while writing user organization invitation lists.
#[derive(Debug, Error)]
pub enum UserOrganizationInvitationListWriterError {
    #[error("user organization invitation list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
