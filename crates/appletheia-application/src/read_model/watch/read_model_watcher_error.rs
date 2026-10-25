use std::error::Error as StdError;

use thiserror::Error;

use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{ReadModelWatchEndpointError, ReadModelWatchRegistryError};

#[derive(Debug, Error)]
pub enum ReadModelWatcherError {
    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    Registry(#[from] ReadModelWatchRegistryError),

    #[error(transparent)]
    Endpoint(#[from] ReadModelWatchEndpointError),

    #[error("watch registration failed: {operation_error}; local cleanup failed: {cleanup_error}")]
    OperationAndCleanupFailed {
        #[source]
        operation_error: Box<ReadModelWatcherError>,
        cleanup_error: ReadModelWatchEndpointError,
    },

    #[error("read model watcher backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
