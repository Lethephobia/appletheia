use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyMintAccountCreationSagaError {
    #[error("unexpected currency mint account creation saga event")]
    UnexpectedEvent,
}
