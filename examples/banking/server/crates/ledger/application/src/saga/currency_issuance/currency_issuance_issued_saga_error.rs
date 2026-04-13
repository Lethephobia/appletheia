use appletheia::application::command::CommandRequestOwnedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyIssuanceIssuedSagaError {
    #[error("unexpected currency issuance issued saga event")]
    UnexpectedEvent,
    #[error(transparent)]
    CommandRequest(#[from] CommandRequestOwnedError),
}
