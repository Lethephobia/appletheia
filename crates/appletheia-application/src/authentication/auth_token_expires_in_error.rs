use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenExpiresInError {
    #[error("expires_in must be positive")]
    NonPositive,
}
