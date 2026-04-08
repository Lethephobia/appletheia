use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Stores progress for the currency issuance orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceSagaState {
    pub currency_definition_id: Option<CurrencyDefinitionId>,
    pub destination_account_id: Option<AccountId>,
    pub amount: Option<CurrencyAmount>,
    pub currency_issuance_id: Option<CurrencyIssuanceId>,
}

impl SagaState for CurrencyIssuanceSagaState {}
