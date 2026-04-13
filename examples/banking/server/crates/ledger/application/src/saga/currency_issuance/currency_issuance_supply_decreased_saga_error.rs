use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyIssuanceSupplyDecreasedSagaError {
    #[error("unexpected currency issuance supply decreased saga event")]
    UnexpectedEvent,
    #[error("currency issuance supply decreased saga context is required")]
    ContextRequired,
}
