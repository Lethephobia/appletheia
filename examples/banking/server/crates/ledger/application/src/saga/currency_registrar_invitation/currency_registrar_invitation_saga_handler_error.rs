use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the currency registrar invitation saga handlers.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
