use banking_ledger_domain::currency::CurrencySymbolChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency symbol.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySymbolChangeOutput {
    Changed,
    Rejected {
        reason: CurrencySymbolChangeRejectionReason,
    },
}
