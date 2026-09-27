use thiserror::Error;

use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::ProjectorProcessedEventStoreError;

#[derive(Debug, Error)]
pub enum ProjectorRunnerError {
    #[error("processed event store failed: {0}")]
    ProcessedEventStore(#[from] ProjectorProcessedEventStoreError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("projection failed")]
    Projection(#[source] Box<dyn std::error::Error + Send + Sync>),
}
