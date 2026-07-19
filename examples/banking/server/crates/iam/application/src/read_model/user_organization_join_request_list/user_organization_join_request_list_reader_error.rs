use std::error::Error;

use thiserror::Error;

/// Error returned while reading user organization join request lists.
#[derive(Debug, Error)]
pub enum UserOrganizationJoinRequestListReaderError {
    #[error("user organization join request list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
