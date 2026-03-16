use std::error::Error;

use thiserror::Error;

use crate::messaging::ConsumerError;
use crate::messaging::SubscriberError;
use crate::outbox::command::CommandEnvelopeError;

#[derive(Debug, Error)]
pub enum CommandWorkerError {
    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    CommandEnvelope(#[from] CommandEnvelopeError),

    #[error("command dispatch error: {0}")]
    Dispatch(#[source] Box<dyn Error + Send + Sync>),
}
