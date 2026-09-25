use thiserror::Error;

use crate::read_model::PublicOrganizationListReaderError;

/// Error returned while handling public organization list queries.
#[derive(Debug, Error)]
pub enum PublicOrganizationListQueryHandlerError {
    #[error("public organization list reader failed")]
    Reader(#[from] PublicOrganizationListReaderError),
}
