use banking_ledger_domain::account::{
    AccountOwnershipTransferRejectionReason, AccountOwnershipTransferResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after transferring account ownership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountOwnershipTransferOutput {
    Transferred,
    Rejected {
        reason: AccountOwnershipTransferRejectionReason,
    },
}

impl From<AccountOwnershipTransferResult> for AccountOwnershipTransferOutput {
    fn from(value: AccountOwnershipTransferResult) -> Self {
        match value {
            AccountOwnershipTransferResult::Transferred => Self::Transferred,
            AccountOwnershipTransferResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
