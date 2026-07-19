use std::error::Error;

use thiserror::Error;

/// Error returned while loading public organization list read models.
#[derive(Debug, Error)]
pub enum PublicOrganizationListReaderError {
    #[error("public organization list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
