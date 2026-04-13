use appletheia::application::command::CommandRequestOwnedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferRequestedSagaError {
    #[error("unexpected transfer requested saga event")]
    UnexpectedEvent,
    #[error(transparent)]
    CommandRequest(#[from] CommandRequestOwnedError),
}
