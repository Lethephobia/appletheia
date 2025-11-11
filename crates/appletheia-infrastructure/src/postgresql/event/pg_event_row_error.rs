use thiserror::Error;

use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersionError, EventIdError, EventPayload,
};

#[derive(Debug, Error)]
pub enum PgEventRowError<A: Aggregate> {
    #[error("event id error: {0}")]
    EventId(#[source] EventIdError),

    #[error("aggregate id error: {0}")]
    AggregateId(#[source] <A::Id as AggregateId>::Error),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[source] AggregateVersionError),

    #[error("event payload error: {0}")]
    EventPayload(#[source] <A::EventPayload as EventPayload>::Error),
}
