use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization join request lists.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestListWriterError {
    #[error("organization join request list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
