use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use serde::{Deserialize, Serialize};

use banking_ledger_domain::deposit::DepositId;

use crate::settlement::DepositSettlementPreparation;

/// Returned after a deposit settlement transaction is prepared.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositSettlementPrepareOutput {
    pub deposit_id: DepositId,
    pub preparation: DepositSettlementPreparation,
}

impl CommandOutput for DepositSettlementPrepareOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
