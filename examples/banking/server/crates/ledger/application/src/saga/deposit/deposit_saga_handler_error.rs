use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the deposit saga handlers.
#[derive(Debug, Error)]
pub enum DepositSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
