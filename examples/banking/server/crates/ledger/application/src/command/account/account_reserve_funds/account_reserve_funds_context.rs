use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Carries the workflow context for `AccountReserveFundsCommand`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountReserveFundsContext {
    #[default]
    Direct,
    Transfer {
        transfer_id: TransferId,
    },
}
