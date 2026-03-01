use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwksError {
    #[error("invalid jwks json")]
    InvalidJson(#[source] serde_json::Error),
}
