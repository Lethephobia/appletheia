use std::error::Error;

use thiserror::Error;

/// Error returned while loading organization-internal information read models.
#[derive(Debug, Error)]
pub enum OrganizationInternalInfoReaderError {
    #[error("organization internal info reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
