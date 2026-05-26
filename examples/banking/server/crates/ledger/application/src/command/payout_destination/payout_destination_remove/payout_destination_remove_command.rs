use appletheia::command;
use banking_ledger_domain::payout_destination::PayoutDestinationId;
use serde::{Deserialize, Serialize};

/// Removes the specified payout destination.
#[command(name = "payout_destination_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutDestinationRemoveCommand {
    pub payout_destination_id: PayoutDestinationId,
}
