use appletheia::application::saga::SagaState;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

use super::CurrencyProvisioningSagaStatus;

/// Stores state for the currency provisioning saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyProvisioningSagaState {
    pub currency_id: CurrencyId,
    pub status: CurrencyProvisioningSagaStatus,
}

impl CurrencyProvisioningSagaState {
    pub fn new(currency_id: CurrencyId) -> Self {
        Self {
            currency_id,
            status: CurrencyProvisioningSagaStatus::Defined,
        }
    }
}

impl SagaState for CurrencyProvisioningSagaState {}
