use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumerError {
    #[error("consumer next error")]
    Next(#[source] Box<dyn Error + Send + Sync>),
}
