use banking_ledger_domain::payout_destination::PayoutDestinationRemoveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a payout destination removal request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum PayoutDestinationRemoveOutput {
    Removed,
    Rejected {
        reason: PayoutDestinationRemoveRejectionReason,
    },
}
