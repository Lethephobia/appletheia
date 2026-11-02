use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::deposit::DepositId;
use serde::{Deserialize, Serialize};

/// Stores data needed by the deposit orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositSagaState {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
    pub deposit_id: DepositId,
}

impl DepositSagaState {
    pub fn new(deposit_id: DepositId, account_id: AccountId, amount: CurrencyAmount) -> Self {
        Self {
            account_id,
            amount,
            deposit_id,
        }
    }
}

impl SagaState for DepositSagaState {}
