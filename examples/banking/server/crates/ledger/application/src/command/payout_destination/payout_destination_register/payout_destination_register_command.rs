use appletheia::command;
use banking_ledger_domain::payout_destination::{
    PayoutDestinationOwner, PayoutDestinationTokenAccountOwnerAddress,
};
use serde::{Deserialize, Serialize};

/// Registers a payout destination.
#[command(name = "payout_destination_register")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutDestinationRegisterCommand {
    pub owner: PayoutDestinationOwner,
    pub token_account_owner_address: PayoutDestinationTokenAccountOwnerAddress,
}
