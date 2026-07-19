use std::error::Error;

use thiserror::Error;

/// Error returned while writing public organization list read models.
#[derive(Debug, Error)]
pub enum PublicOrganizationListWriterError {
    #[error("public organization list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
