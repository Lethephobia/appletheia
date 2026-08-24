use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use serde::{Deserialize, Serialize};

use banking_ledger_domain::deposit::{DepositId, DepositRequestRejectionReason};

use crate::settlement::DepositSettlementPreparation;

/// Returned after a deposit settlement transaction is prepared.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositSettlementPrepareOutput {
    Prepared {
        deposit_id: DepositId,
        preparation: DepositSettlementPreparation,
    },
    Rejected {
        deposit_id: DepositId,
        reason: DepositRequestRejectionReason,
    },
}

impl CommandOutput for DepositSettlementPrepareOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
