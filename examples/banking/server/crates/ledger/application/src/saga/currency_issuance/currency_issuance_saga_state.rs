use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

use super::CurrencyIssuanceSagaStatus;

/// Stores progress for the currency issuance orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceSagaState {
    pub currency_id: CurrencyId,
    pub destination_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub currency_issuance_id: CurrencyIssuanceId,
    pub status: CurrencyIssuanceSagaStatus,
}

impl CurrencyIssuanceSagaState {
    pub fn new(
        currency_issuance_id: CurrencyIssuanceId,
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            currency_id,
            destination_account_id,
            amount,
            currency_issuance_id,
            status: CurrencyIssuanceSagaStatus::SupplyReserveRequested,
        }
    }
}

impl SagaState for CurrencyIssuanceSagaState {}
