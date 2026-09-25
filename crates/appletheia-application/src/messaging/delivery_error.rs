use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeliveryError {
    #[error("delivery ack error")]
    Ack(#[source] Box<dyn Error + Send + Sync>),

    #[error("delivery nack error")]
    Nack(#[source] Box<dyn Error + Send + Sync>),
}
