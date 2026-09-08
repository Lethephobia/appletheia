use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the transfer saga handlers.
#[derive(Debug, Error)]
pub enum TransferSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
