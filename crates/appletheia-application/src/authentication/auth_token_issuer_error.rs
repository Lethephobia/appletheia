use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenIssuerError {
    #[error("token issue failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
