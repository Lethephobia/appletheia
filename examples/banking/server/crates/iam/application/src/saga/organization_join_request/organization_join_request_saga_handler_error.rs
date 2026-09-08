use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the organization join request saga handlers.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
