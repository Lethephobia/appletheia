use thiserror::Error;

use appletheia_domain::{AggregateId, AggregateVersionError, EventIdError, EventPayload};

#[derive(Debug, Error)]
pub enum PgEventRowError<I: AggregateId, P: EventPayload> {
    #[error("event id error: {0}")]
    EventId(#[source] EventIdError),

    #[error("aggregate id error: {0}")]
    AggregateId(#[source] I::Error),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[source] AggregateVersionError),

    #[error("event payload error: {0}")]
    EventPayload(#[source] P::Error),
}
