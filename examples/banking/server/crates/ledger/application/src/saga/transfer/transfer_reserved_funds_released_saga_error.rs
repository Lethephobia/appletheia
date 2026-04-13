use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferReservedFundsReleasedSagaError {
    #[error("unexpected transfer reserved funds released saga event")]
    UnexpectedEvent,
    #[error("transfer reserved funds released saga context is required")]
    ContextRequired,
}
