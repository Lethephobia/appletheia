use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumerBuilderError {
    #[error("consumer subscribe error")]
    Subscribe(#[source] Box<dyn Error + Send + Sync>),
}

