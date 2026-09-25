use appletheia_application::command::CommandNameOwnedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum PgSagaDispatchedCommandRowError {
    #[error(transparent)]
    CommandName(#[from] CommandNameOwnedError),
    #[error(transparent)]
    StepDeserialize(#[from] serde_json::Error),
}
