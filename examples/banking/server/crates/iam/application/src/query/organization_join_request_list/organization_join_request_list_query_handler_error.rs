use thiserror::Error;

use crate::read_model::OrganizationJoinRequestListReaderError;

/// Error returned while handling organization join request list queries.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestListQueryHandlerError {
    #[error("organization join request list reader failed")]
    Reader(#[from] OrganizationJoinRequestListReaderError),
}
