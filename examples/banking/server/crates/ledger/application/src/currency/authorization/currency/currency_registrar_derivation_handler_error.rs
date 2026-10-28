use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::currency::CurrencyError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    Currency(#[from] CurrencyError),
}
