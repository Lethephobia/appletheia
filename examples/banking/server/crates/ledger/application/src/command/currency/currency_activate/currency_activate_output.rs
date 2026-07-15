use banking_ledger_domain::currency::CurrencyActivateRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a currency activation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyActivateOutput {
    Activated,
    Rejected {
        reason: CurrencyActivateRejectionReason,
    },
}
