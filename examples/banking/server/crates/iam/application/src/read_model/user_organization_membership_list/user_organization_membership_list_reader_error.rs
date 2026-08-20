use std::error::Error;

use thiserror::Error;

/// Error returned while reading user organization membership lists.
#[derive(Debug, Error)]
pub enum UserOrganizationMembershipListReaderError {
    #[error("user organization membership list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
