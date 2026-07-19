use thiserror::Error;

use crate::read_model::UserOrganizationJoinRequestListReaderError;

/// Error returned while handling user organization join request list queries.
#[derive(Debug, Error)]
pub enum UserOrganizationJoinRequestListQueryHandlerError {
    #[error("user organization join request list reader failed")]
    Reader(#[from] UserOrganizationJoinRequestListReaderError),
}
