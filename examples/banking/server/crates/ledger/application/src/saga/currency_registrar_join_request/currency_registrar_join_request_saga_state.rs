use appletheia::application::saga::SagaState;
use banking_ledger_domain::CurrencyRegistrarJoinRequestId;
use serde::{Deserialize, Serialize};

use super::CurrencyRegistrarJoinRequestSagaStatus;

/// Stores the progress of the currency registrar join request saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestSagaState {
    pub currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
    pub status: CurrencyRegistrarJoinRequestSagaStatus,
}

impl CurrencyRegistrarJoinRequestSagaState {
    pub fn new(currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId) -> Self {
        Self {
            currency_registrar_join_request_id,
            status: CurrencyRegistrarJoinRequestSagaStatus::MembershipCreateRequested,
        }
    }
}

impl SagaState for CurrencyRegistrarJoinRequestSagaState {}
