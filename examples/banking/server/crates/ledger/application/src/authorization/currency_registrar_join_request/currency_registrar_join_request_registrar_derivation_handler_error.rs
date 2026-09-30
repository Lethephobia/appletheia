use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::currency_registrar_join_request::CurrencyRegistrarJoinRequestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestRegistrarDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    CurrencyRegistrarJoinRequest(#[from] CurrencyRegistrarJoinRequestError),
}
