use thiserror::Error;

#[derive(Debug, Error)]
pub enum TenantIdError {
    #[error("tenant id must be a valid uuid: {value}")]
    InvalidUuid {
        value: String,
        #[source]
        source: uuid::Error,
    },
}
