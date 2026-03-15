use std::error::Error;

use thiserror::Error;

use appletheia_domain::EventId;

use crate::authorization::AuthorizerError;
use crate::command::CommandFailureReport;
use crate::command::CommandHasherError;
use crate::command::IdempotencyServiceError;
use crate::event::EventSequenceLookupError;
use crate::projection::{
    ProjectorNameOwned, ProjectorProcessedEventStoreError, ReadYourWritesTimeout,
    ReadYourWritesWaitError,
};
use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWorkError;
use crate::unit_of_work::UnitOfWorkFactoryError;

#[derive(Debug, Error)]
pub enum CommandDispatcherError<HE>
where
    HE: Error + Send + Sync + 'static,
{
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

    #[error("idempotency error: {0}")]
    Idempotency(#[from] IdempotencyServiceError),

    #[error("command handler error: {0}")]
    Handler(#[source] HE),

    #[error("command is still in progress: {message_id}")]
    InProgress { message_id: MessageId },

    #[error("previous command failed: {0}")]
    PreviousFailure(CommandFailureReport),

    #[error("command hasher error: {0}")]
    Hasher(#[from] CommandHasherError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("authorizer error: {0}")]
    Authorizer(#[from] AuthorizerError),
}

impl<HE> From<ReadYourWritesWaitError> for CommandDispatcherError<HE>
where
    HE: Error + Send + Sync + 'static,
{
    fn from(value: ReadYourWritesWaitError) -> Self {
        match value {
            ReadYourWritesWaitError::UnitOfWorkFactory(error) => Self::UnitOfWorkFactory(error),
            ReadYourWritesWaitError::UnitOfWork(error) => Self::UnitOfWork(error),
            ReadYourWritesWaitError::EventSequenceLookup(error) => Self::EventSequenceLookup(error),
            ReadYourWritesWaitError::ProjectorProcessedEventStore(error) => {
                Self::ProjectorProcessedEventStore(error)
            }
            ReadYourWritesWaitError::UnknownMessageId { message_id } => {
                Self::UnknownMessageId { message_id }
            }
            ReadYourWritesWaitError::Timeout {
                target_event_id,
                pending,
                timeout,
            } => Self::Timeout {
                target_event_id,
                pending,
                timeout,
            },
        }
    }
}
