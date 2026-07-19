use std::error::Error;

use thiserror::Error;

/// Error returned while writing organization-management information read models.
#[derive(Debug, Error)]
pub enum OrganizationManagementInfoWriterError {
    #[error("organization management info writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
