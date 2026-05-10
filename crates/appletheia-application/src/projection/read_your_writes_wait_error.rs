use thiserror::Error;

use crate::event::EventLookupError;
use crate::projection::ProjectorNameOwned;
use crate::projection::ProjectorProcessedEventStoreError;
use crate::request_context::MessageId;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::ReadYourWritesTimeout;

#[derive(Debug, Error)]
pub enum ReadYourWritesWaitError {
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

    #[error(
        "read-your-writes timed out (message_id={message_id}, pending_projectors={pending_projectors:?}, timeout={timeout:?})"
    )]
    Timeout {
        message_id: MessageId,
        pending_projectors: Vec<ProjectorNameOwned>,
        timeout: ReadYourWritesTimeout,
    },
}
