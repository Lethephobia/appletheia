use serde::{Deserialize, Serialize};

use banking_ledger_domain::deposit::{DepositId, DepositRequestRejectionReason};

use crate::mint::TokenDepositPreparation;

/// Returned after a deposit token transfer transaction is prepared.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositTokenTransferPrepareOutput {
    Prepared {
        deposit_id: DepositId,
        preparation: TokenDepositPreparation,
    },
    Rejected {
        deposit_id: DepositId,
        reason: DepositRequestRejectionReason,
    },
}
