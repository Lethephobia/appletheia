use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::deposit::DepositId;
use serde::{Deserialize, Serialize};

use super::DepositSagaStatus;

/// Stores progress for the deposit orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositSagaState {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
    pub deposit_id: DepositId,
    pub status: DepositSagaStatus,
}

impl DepositSagaState {
    pub fn new(deposit_id: DepositId, account_id: AccountId, amount: CurrencyAmount) -> Self {
        Self {
            account_id,
            amount,
            deposit_id,
            status: DepositSagaStatus::AccountDepositRequested,
        }
    }
}

impl SagaState for DepositSagaState {}
