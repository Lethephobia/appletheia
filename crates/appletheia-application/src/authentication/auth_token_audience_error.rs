use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenAudienceError {
    #[error("audience must not be empty")]
    Empty,
}
