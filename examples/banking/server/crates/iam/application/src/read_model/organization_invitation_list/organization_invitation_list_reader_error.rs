use std::error::Error;

use thiserror::Error;

/// Error returned while reading organization invitation lists.
#[derive(Debug, Error)]
pub enum OrganizationInvitationListReaderError {
    #[error("organization invitation list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
