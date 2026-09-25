use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization fragment read models.
#[derive(Debug, Error)]
pub enum OrganizationFragmentWriterError {
    #[error("organization fragment writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
