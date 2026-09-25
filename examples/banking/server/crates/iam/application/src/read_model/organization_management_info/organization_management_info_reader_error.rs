use std::error::Error;

use thiserror::Error;

/// Error returned while loading organization-management information read models.
#[derive(Debug, Error)]
pub enum OrganizationManagementInfoReaderError {
    #[error("organization management info reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
