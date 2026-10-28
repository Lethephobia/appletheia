use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembershipError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipRegistrarDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    CurrencyRegistrarMembership(#[from] CurrencyRegistrarMembershipError),
}
