use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the owned account closure saga handlers.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),

    #[error("owned account closure saga state is missing closure id")]
    MissingOwnedAccountClosureId,
}
