use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Stores context for the currency issuance orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceSagaContext {
    pub currency_id: CurrencyId,
    pub destination_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub currency_issuance_id: CurrencyIssuanceId,
    pub status: CurrencyIssuanceSagaStatus,
}

impl CurrencyIssuanceSagaContext {
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
            status: CurrencyIssuanceSagaStatus::Issued,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyIssuanceSagaStatus {
    #[default]
    Initial,
    Issued,
    SupplyIncreased,
    Deposited,
    SupplyDecreased,
    Completed,
    Failed,
}
