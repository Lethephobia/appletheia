use thiserror::Error;

use crate::read_model::OrganizationManagementInfoReaderError;

/// Error returned while handling organization-management information queries.
#[derive(Debug, Error)]
pub enum OrganizationManagementInfoQueryHandlerError {
    #[error("organization management info reader failed")]
    Reader(#[from] OrganizationManagementInfoReaderError),
}
