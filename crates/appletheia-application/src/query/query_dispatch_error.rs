use std::error::Error;

use thiserror::Error as ThisError;

use crate::event::EventSequence;
use crate::event::EventSequenceLookupError;
use crate::projection::{ProjectionCheckpointStoreError, ProjectorNameOwned};
use crate::request_context::MessageId;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::ReadYourWritesTimeout;

#[derive(Debug, ThisError)]
pub enum QueryDispatchError<HE>
where
    HE: Error + Send + Sync + 'static,
{
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("event sequence lookup error: {0}")]
    EventSequenceLookup(#[from] EventSequenceLookupError),

    #[error("projection checkpoint store error: {0}")]
    ProjectionCheckpointStore(#[from] ProjectionCheckpointStoreError),

    #[error("no event sequence found for message id: {message_id}")]
    UnknownMessageId { message_id: MessageId },

    #[error(
        "read-your-writes timed out (projector={projector_name}, target={target}, last_checkpoint={last_checkpoint:?}, timeout={timeout:?})"
    )]
    Timeout {
        projector_name: ProjectorNameOwned,
        target: EventSequence,
        last_checkpoint: Option<EventSequence>,
        timeout: ReadYourWritesTimeout,
    },

    #[error("query handler error: {0}")]
    Handler(#[source] HE),
}
