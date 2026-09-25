use std::error::Error;

use thiserror::Error;

/// Error returned while reading organization join request lists.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestListReaderError {
    #[error("organization join request list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
