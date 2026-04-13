use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyIssuanceDepositedSagaError {
    #[error("unexpected currency issuance deposited saga event")]
    UnexpectedEvent,
    #[error("currency issuance deposited saga context is required")]
    ContextRequired,
}
