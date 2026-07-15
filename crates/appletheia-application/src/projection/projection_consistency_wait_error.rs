use thiserror::Error;

use crate::event::EventLookupError;
use crate::projection::ProjectorNameOwned;
use crate::projection::ProjectorProcessedEventStoreError;
use crate::request_context::MessageId;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};
use appletheia_domain::EventId;

use super::ProjectionConsistencyTimeout;

#[derive(Debug, Error)]
pub enum ProjectionConsistencyWaitError {
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("event lookup error: {0}")]
    EventLookup(#[from] EventLookupError),

    #[error("projector processed event store error: {0}")]
    ProjectorProcessedEventStore(#[from] ProjectorProcessedEventStoreError),

    #[error("no event found for message id: {message_id}")]
    UnknownMessageId { message_id: MessageId },

    #[error("no event found for event ids: {event_ids:?}")]
    UnknownEventIds { event_ids: Vec<EventId> },

    #[error(
        "projection consistency timed out (pending_projectors={pending_projectors:?}, timeout={timeout:?})"
    )]
    Timeout {
        pending_projectors: Vec<ProjectorNameOwned>,
        timeout: ProjectionConsistencyTimeout,
    },
}
