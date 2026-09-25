use thiserror::Error;

use crate::read_model::UserOrganizationInvitationListReaderError;

/// Error returned while handling user organization invitation list queries.
#[derive(Debug, Error)]
pub enum UserOrganizationInvitationListQueryHandlerError {
    #[error("user organization invitation list reader failed")]
    Reader(#[from] UserOrganizationInvitationListReaderError),
}
