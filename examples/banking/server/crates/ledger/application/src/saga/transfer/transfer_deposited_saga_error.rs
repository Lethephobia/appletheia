use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferDepositedSagaError {
    #[error("unexpected transfer deposited saga event")]
    UnexpectedEvent,
    #[error("transfer deposited saga context is required")]
    ContextRequired,
}
