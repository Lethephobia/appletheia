use appletheia::application::command::CommandRequestOwnedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyIssuanceSupplyIncreasedSagaError {
    #[error("unexpected currency issuance supply increased saga event")]
    UnexpectedEvent,
    #[error("currency issuance supply increased saga context is required")]
    ContextRequired,
    #[error(transparent)]
    CommandRequest(#[from] CommandRequestOwnedError),
}
