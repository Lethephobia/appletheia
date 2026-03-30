use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Carries the workflow context for `AccountCommitReservedFundsCommand`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountCommitReservedFundsContext {
    #[default]
    Direct,
    Transfer {
        transfer_id: TransferId,
    },
}
