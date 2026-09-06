use appletheia::application::saga::SagaState;
use banking_ledger_domain::CurrencyRegistrarJoinRequestId;
use serde::{Deserialize, Serialize};

/// Stores data needed by the currency registrar join request saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestSagaState {
    pub currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
}

impl CurrencyRegistrarJoinRequestSagaState {
    pub fn new(currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId) -> Self {
        Self {
            currency_registrar_join_request_id,
        }
    }
}

impl SagaState for CurrencyRegistrarJoinRequestSagaState {}
