use thiserror::Error;

use crate::read_model::OrganizationMemberListReaderError;

/// Error returned while handling organization member list queries.
#[derive(Debug, Error)]
pub enum OrganizationMemberListQueryHandlerError {
    #[error("organization member list reader failed")]
    Reader(#[from] OrganizationMemberListReaderError),
}
