use appletheia::application::saga::SagaState;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

use super::CurrencyMintAccountMetadataSyncSagaStatus;

/// Stores state for the currency mint account metadata sync saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountMetadataSyncSagaState {
    pub currency_id: CurrencyId,
    pub status: CurrencyMintAccountMetadataSyncSagaStatus,
}

impl CurrencyMintAccountMetadataSyncSagaState {
    pub fn new(currency_id: CurrencyId) -> Self {
        Self {
            currency_id,
            status: CurrencyMintAccountMetadataSyncSagaStatus::SyncRequested,
        }
    }
}

impl SagaState for CurrencyMintAccountMetadataSyncSagaState {}
