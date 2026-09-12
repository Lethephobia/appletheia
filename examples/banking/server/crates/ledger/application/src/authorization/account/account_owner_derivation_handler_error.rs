use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::account::AccountError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountOwnerDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Account(#[from] AccountError),
}
