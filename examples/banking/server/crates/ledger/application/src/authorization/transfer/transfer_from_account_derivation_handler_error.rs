use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::transfer::TransferError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferFromAccountDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Transfer(#[from] TransferError),
}
