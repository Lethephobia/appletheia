use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::currency::{CurrencyProvisionRejectionReason, MintAccount};
use serde::{Deserialize, Serialize};

/// Returned after attempting to provision a currency.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyProvisionOutput {
    Provisioned {
        mint_account: MintAccount,
    },
    Rejected {
        reason: CurrencyProvisionRejectionReason,
    },
}

impl CommandOutput for CurrencyProvisionOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
