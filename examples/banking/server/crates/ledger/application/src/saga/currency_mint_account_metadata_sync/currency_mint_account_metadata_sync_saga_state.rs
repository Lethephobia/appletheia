use appletheia::application::saga::SagaState;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Stores state for the currency mint account metadata sync saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountMetadataSyncSagaState {
    pub currency_id: CurrencyId,
}

impl CurrencyMintAccountMetadataSyncSagaState {
    pub fn new(currency_id: CurrencyId) -> Self {
        Self { currency_id }
    }
}

impl SagaState for CurrencyMintAccountMetadataSyncSagaState {}
