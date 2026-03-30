use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Carries the workflow context for `AccountReleaseReservedFundsCommand`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountReleaseReservedFundsContext {
    #[default]
    Direct,
    Transfer {
        transfer_id: TransferId,
    },
}
