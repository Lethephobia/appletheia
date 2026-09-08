use crate::event::EventEnvelopeError;
use thiserror::Error;

/// Reports decoding, routing, or application callback failures.
#[derive(Debug, Error)]
pub enum SagaRouteError<E: std::error::Error + Send + Sync + 'static> {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),
    #[error("event payload name does not match its envelope")]
    EventNameMismatch,
    #[error("saga handler failed")]
    Handler(#[source] E),
}
