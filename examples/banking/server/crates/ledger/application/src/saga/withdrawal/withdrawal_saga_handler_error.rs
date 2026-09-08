use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the withdrawal saga handlers.
#[derive(Debug, Error)]
pub enum WithdrawalSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
