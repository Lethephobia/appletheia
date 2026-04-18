use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectUploadSignerError {
    #[error("object upload signing failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
