use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the currency registrar join request saga handlers.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
