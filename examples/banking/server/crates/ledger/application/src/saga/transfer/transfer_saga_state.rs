use appletheia::application::saga::SagaState;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

use super::TransferSagaStatus;

/// Stores progress for the transfer orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSagaState {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub transfer_id: TransferId,
    pub status: TransferSagaStatus,
}

impl TransferSagaState {
    pub fn new(
        transfer_id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
            status: TransferSagaStatus::Requested,
        }
    }
}

impl SagaState for TransferSagaState {}
