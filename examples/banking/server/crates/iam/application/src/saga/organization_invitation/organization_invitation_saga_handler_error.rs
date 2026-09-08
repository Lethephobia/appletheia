use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the organization invitation saga handlers.
#[derive(Debug, Error)]
pub enum OrganizationInvitationSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
