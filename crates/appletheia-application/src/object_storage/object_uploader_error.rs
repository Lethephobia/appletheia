use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectUploaderError {
    #[error("object uploader backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
