use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenVerifyError {
    #[error("token verify failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
