use thiserror::Error;

use crate::read_model::UserOrganizationMembershipListReaderError;

/// Error returned while handling user organization membership list queries.
#[derive(Debug, Error)]
pub enum UserOrganizationMembershipListQueryHandlerError {
    #[error("user organization membership list reader failed")]
    Reader(#[from] UserOrganizationMembershipListReaderError),
}
