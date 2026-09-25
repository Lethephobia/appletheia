use std::error::Error;

use thiserror::Error;

/// Error returned while reading user organization invitation lists.
#[derive(Debug, Error)]
pub enum UserOrganizationInvitationListReaderError {
    #[error("user organization invitation list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
