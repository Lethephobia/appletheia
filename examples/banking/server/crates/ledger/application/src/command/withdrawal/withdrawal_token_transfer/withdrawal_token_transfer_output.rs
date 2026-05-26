use banking_ledger_domain::withdrawal::WithdrawalOnchainTransactionId;
use serde::{Deserialize, Serialize};

/// Returned after an external withdrawal token transfer attempt is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalTokenTransferOutput {
    TokenTransferred {
        onchain_transaction_id: WithdrawalOnchainTransactionId,
    },
    Rejected,
}
