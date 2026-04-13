use appletheia::application::command::CommandRequestOwnedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferFundsReservedSagaError {
    #[error("unexpected transfer funds reserved saga event")]
    UnexpectedEvent,
    #[error("transfer funds reserved saga context is required")]
    ContextRequired,
    #[error(transparent)]
    CommandRequest(#[from] CommandRequestOwnedError),
}
