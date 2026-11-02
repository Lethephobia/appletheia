use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::token_binding::TokenBindingError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBindingCurrencyDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    TokenBinding(#[from] TokenBindingError),
}
