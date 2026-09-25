use thiserror::Error;

use crate::read_model::OrganizationInvitationListReaderError;

/// Error returned while handling organization invitation list queries.
#[derive(Debug, Error)]
pub enum OrganizationInvitationListQueryHandlerError {
    #[error("organization invitation list reader failed")]
    Reader(#[from] OrganizationInvitationListReaderError),
}
