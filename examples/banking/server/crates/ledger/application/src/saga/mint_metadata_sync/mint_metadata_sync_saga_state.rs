use appletheia::application::saga::SagaState;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

use super::MintMetadataSyncSagaStatus;

/// Stores state for the currency mint metadata sync saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintMetadataSyncSagaState {
    pub currency_id: CurrencyId,
    pub status: MintMetadataSyncSagaStatus,
}

impl MintMetadataSyncSagaState {
    pub fn new(currency_id: CurrencyId) -> Self {
        Self {
            currency_id,
            status: MintMetadataSyncSagaStatus::SyncRequested,
        }
    }
}

impl SagaState for MintMetadataSyncSagaState {}
