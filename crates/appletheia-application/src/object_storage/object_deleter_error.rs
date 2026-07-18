use thiserror::Error;

use crate::Retryability;

#[derive(Debug, Error)]
pub enum ObjectDeleterError {
    #[error("object delete failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for ObjectDeleterError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
