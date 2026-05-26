use banking_ledger_domain::payout_destination::PayoutDestinationId;
use serde::{Deserialize, Serialize};

/// Returned after a payout destination registration request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutDestinationRegisterOutput {
    pub payout_destination_id: PayoutDestinationId,
}

impl PayoutDestinationRegisterOutput {
    pub fn new(payout_destination_id: PayoutDestinationId) -> Self {
        Self {
            payout_destination_id,
        }
    }
}
