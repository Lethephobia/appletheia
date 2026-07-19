use std::error::Error;

use thiserror::Error;

/// Error returned while writing user organization join request lists.
#[derive(Debug, Error)]
pub enum UserOrganizationJoinRequestListWriterError {
    #[error("user organization join request list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
