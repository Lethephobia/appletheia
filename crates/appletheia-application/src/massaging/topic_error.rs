use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TopicError {
    #[error("subscribe error")]
    Subscribe(#[source] Box<dyn Error + Send + Sync>),
}
