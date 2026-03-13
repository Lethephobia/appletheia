use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenVerifierError {
    #[error("token verify failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
