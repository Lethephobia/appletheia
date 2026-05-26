use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

use super::WithdrawalSagaStatus;

/// Stores progress for the withdrawal orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalSagaState {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
    pub withdrawal_id: WithdrawalId,
    pub status: WithdrawalSagaStatus,
}

impl WithdrawalSagaState {
    pub fn new(withdrawal_id: WithdrawalId, account_id: AccountId, amount: CurrencyAmount) -> Self {
        Self {
            account_id,
            amount,
            withdrawal_id,
            status: WithdrawalSagaStatus::Requested,
        }
    }
}

impl SagaState for WithdrawalSagaState {}
