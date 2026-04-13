use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferReservedFundsCommittedSagaError {
    #[error("unexpected transfer reserved funds committed saga event")]
    UnexpectedEvent,
    #[error("transfer reserved funds committed saga context is required")]
    ContextRequired,
}
