use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Stores progress for the currency issuance orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceSagaState {
    pub currency_definition_id: CurrencyDefinitionId,
    pub destination_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub currency_issuance_id: CurrencyIssuanceId,
    pub status: CurrencyIssuanceSagaStatus,
}

impl CurrencyIssuanceSagaState {
    pub fn new(
        currency_issuance_id: CurrencyIssuanceId,
        currency_definition_id: CurrencyDefinitionId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            currency_definition_id,
            destination_account_id,
            amount,
            currency_issuance_id,
            status: CurrencyIssuanceSagaStatus::Issued,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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

impl SagaState for CurrencyIssuanceSagaState {}
