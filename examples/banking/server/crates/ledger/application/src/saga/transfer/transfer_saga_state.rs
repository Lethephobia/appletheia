use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Stores progress for the transfer orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSagaState {
    pub from_account_id: Option<AccountId>,
    pub to_account_id: Option<AccountId>,
    pub amount: Option<CurrencyAmount>,
    pub transfer_id: Option<TransferId>,
}

impl SagaState for TransferSagaState {}
