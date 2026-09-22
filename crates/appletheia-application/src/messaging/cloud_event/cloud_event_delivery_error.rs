use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventDeliveryError {
    #[error("CloudEvent delivery ack error")]
    Ack(#[source] Box<dyn Error + Send + Sync>),

    #[error("CloudEvent delivery nack error")]
    Nack(#[source] Box<dyn Error + Send + Sync>),
}
