use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::withdrawal::WithdrawalError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WithdrawalAccountDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Withdrawal(#[from] WithdrawalError),
}
