use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectDeleterError {
    #[error("object delete failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
