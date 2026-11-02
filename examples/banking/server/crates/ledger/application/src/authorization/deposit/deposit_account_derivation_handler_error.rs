use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::deposit::DepositError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepositAccountDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Deposit(#[from] DepositError),
}
