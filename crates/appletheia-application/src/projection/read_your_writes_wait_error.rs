use thiserror::Error as ThisError;

use crate::event::EventLookupError;
use crate::projection::ProjectorNameOwned;
use crate::projection::ProjectorProcessedEventStoreError;
use crate::request_context::{CorrelationId, MessageId};
use crate::saga::{SagaNameOwned, SagaStatusLookupError};
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{ReadYourWritesTarget, ReadYourWritesTimeout};

#[derive(Debug, ThisError)]
pub enum ReadYourWritesWaitError {
    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("event lookup error: {0}")]
    EventLookup(#[from] EventLookupError),

    #[error("saga status lookup error: {0}")]
    SagaStatusLookup(#[from] SagaStatusLookupError),

    #[error("projector processed event store error: {0}")]
    ProjectorProcessedEventStore(#[from] ProjectorProcessedEventStoreError),

    #[error("no event found for message id: {message_id}")]
    UnknownMessageId { message_id: MessageId },

    #[error("no event found for correlation id: {correlation_id}")]
    UnknownCorrelationId { correlation_id: CorrelationId },

    #[error(
        "read-your-writes timed out (target={target:?}, pending_projectors={pending_projectors:?}, pending_sagas={pending_sagas:?}, timeout={timeout:?})"
    )]
    Timeout {
        target: ReadYourWritesTarget,
        pending_projectors: Vec<ProjectorNameOwned>,
        pending_sagas: Vec<SagaNameOwned>,
        timeout: ReadYourWritesTimeout,
    },
}
