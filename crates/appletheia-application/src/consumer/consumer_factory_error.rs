use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumerFactoryError {
    #[error("consumer subscribe error")]
    Subscribe(#[source] Box<dyn Error + Send + Sync>),
}
