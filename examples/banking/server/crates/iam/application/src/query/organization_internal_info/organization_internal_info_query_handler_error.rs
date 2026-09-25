use thiserror::Error;

use crate::read_model::OrganizationInternalInfoReaderError;

/// Error returned while handling organization-internal information queries.
#[derive(Debug, Error)]
pub enum OrganizationInternalInfoQueryHandlerError {
    #[error("organization internal info reader failed")]
    Reader(#[from] OrganizationInternalInfoReaderError),
}
