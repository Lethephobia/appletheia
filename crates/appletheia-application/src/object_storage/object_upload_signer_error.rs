use thiserror::Error;

use crate::Retryability;

#[derive(Debug, Error)]
pub enum ObjectUploadSignerError {
    #[error("object upload signing failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for ObjectUploadSignerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
