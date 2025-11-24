use thiserror::Error;

use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersionError, EventIdError, EventPayload,
};

#[derive(Debug, Error)]
pub enum PgEventRowError<A: Aggregate> {
    #[error("event id error: {0}")]
    EventId(#[from] EventIdError),

    #[error("aggregate id error: {0}")]
    AggregateId(#[source] <A::Id as AggregateId>::Error),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[from] AggregateVersionError),

    #[error("event payload error: {0}")]
    EventPayload(#[source] <A::EventPayload as EventPayload>::Error),
}
