use appletheia::command;
use banking_ledger_domain::core::OnchainTransactionId;
use banking_ledger_domain::deposit::DepositId;
use serde::{Deserialize, Serialize};

/// Records an verified token settlement for a deposit.
#[command(name = "deposit_verify_settlement")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositVerifySettlementCommand {
    pub deposit_id: DepositId,
    pub transaction_id: OnchainTransactionId,
}
