use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectUploadExpiresInError {
    #[error("object storage upload expiration must be positive")]
    NonPositive,
}
