use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization-internal information read models.
#[derive(Debug, Error)]
pub enum OrganizationInternalInfoWriterError {
    #[error("organization internal info writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
