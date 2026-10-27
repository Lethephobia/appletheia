use std::error::Error as StdError;

use thiserror::Error;

use crate::Retryability;
use crate::query::QueryDispatcherError;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{
    ReadModelWatchEndpointError, ReadModelWatchGenerationRegistrarError,
    ReadModelWatchGenerationRegistrarFactoryError, ReadModelWatchReceiverError,
    ReadModelWatchRegistryError,
};

#[derive(Debug, Error)]
pub enum ReadModelWatchError<HE>
where
    HE: Retryability,
{
    #[error(transparent)]
    QueryDispatcher(#[from] QueryDispatcherError<HE>),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    Registry(#[from] ReadModelWatchRegistryError),

    #[error(transparent)]
    Endpoint(#[from] ReadModelWatchEndpointError),

    #[error(transparent)]
    Receiver(#[from] ReadModelWatchReceiverError),

    #[error(transparent)]
    GenerationRegistrarFactory(#[from] ReadModelWatchGenerationRegistrarFactoryError),

    #[error(transparent)]
    GenerationRegistrar(#[from] ReadModelWatchGenerationRegistrarError),

    #[error("read model watch subscription backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
