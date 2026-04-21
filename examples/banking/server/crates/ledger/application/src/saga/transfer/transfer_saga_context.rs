use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Stores context for the transfer orchestration saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSagaContext {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub transfer_id: TransferId,
    pub status: TransferSagaStatus,
}

impl TransferSagaContext {
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferSagaStatus {
    #[default]
    Initial,
    Requested,
    FundsReserved,
    Deposited,
    ReservedFundsCommitted,
    ReservedFundsReleased,
    Completed,
    Failed,
}
