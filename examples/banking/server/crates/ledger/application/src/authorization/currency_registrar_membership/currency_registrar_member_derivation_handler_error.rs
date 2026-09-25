use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::CurrencyRegistrarMembershipError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarMemberDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    CurrencyRegistrarMembership(#[from] CurrencyRegistrarMembershipError),
}
