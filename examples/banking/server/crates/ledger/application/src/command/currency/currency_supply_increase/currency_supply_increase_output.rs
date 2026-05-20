use banking_ledger_domain::currency::CurrencySupplyIncreaseRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after increasing currency supply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySupplyIncreaseOutput {
    Increased,
    Rejected {
        reason: CurrencySupplyIncreaseRejectionReason,
    },
}
