use appletheia::command;
use banking_ledger_domain::deposit::DepositId;
use serde::{Deserialize, Serialize};

/// Records an observed external token transfer for a deposit.
#[command(name = "deposit_token_transfer_record")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositTokenTransferRecordCommand {
    pub deposit_id: DepositId,
}
