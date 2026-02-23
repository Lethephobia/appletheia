use thiserror::Error as ThisError;

use appletheia_domain::EventId;

use crate::event::EventSequenceLookupError;
use crate::projection::ProjectorNameOwned;
use crate::projection::ProjectorProcessedEventStoreError;
use crate::request_context::MessageId;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::ReadYourWritesTimeout;

#[derive(Debug, ThisError)]
pub enum ReadYourWritesWaitError {
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("event sequence lookup error: {0}")]
    EventSequenceLookup(#[from] EventSequenceLookupError),

    #[error("projector processed event store error: {0}")]
    ProjectorProcessedEventStore(#[from] ProjectorProcessedEventStoreError),

    #[error("no event found for message id: {message_id}")]
    UnknownMessageId { message_id: MessageId },

    #[error(
        "read-your-writes timed out (target_event_id={target_event_id}, pending={pending:?}, timeout={timeout:?})"
    )]
    Timeout {
        target_event_id: EventId,
        pending: Vec<ProjectorNameOwned>,
        timeout: ReadYourWritesTimeout,
    },
}
