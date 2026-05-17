use appletheia::application::saga::SagaState;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Stores state for the currency old image object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyOldImageObjectDeletionSagaState {
    pub currency_id: CurrencyId,
}

impl CurrencyOldImageObjectDeletionSagaState {
    pub fn new(currency_id: CurrencyId) -> Self {
        Self { currency_id }
    }
}

impl SagaState for CurrencyOldImageObjectDeletionSagaState {}
