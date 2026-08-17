use std::error::Error;

use thiserror::Error;

/// Reports a client transport failure while delivering a routed watch change.
#[derive(Debug, Error)]
#[error("read model watch delivery failed")]
pub struct ReadModelWatchDeliveryError(#[source] Box<dyn Error + Send + Sync>);

impl ReadModelWatchDeliveryError {
    pub fn new(source: impl Error + Send + Sync + 'static) -> Self {
        Self(Box::new(source))
    }
}
