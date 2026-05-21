use appletheia::application::saga::SagaState;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

use super::CurrencyMintAccountCreationSagaStatus;

/// Stores state for the currency mint account creation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountCreationSagaState {
    pub currency_id: CurrencyId,
    pub status: CurrencyMintAccountCreationSagaStatus,
}

impl CurrencyMintAccountCreationSagaState {
    pub fn new(currency_id: CurrencyId) -> Self {
        Self {
            currency_id,
            status: CurrencyMintAccountCreationSagaStatus::Defined,
        }
    }
}

impl SagaState for CurrencyMintAccountCreationSagaState {}
