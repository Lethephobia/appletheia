use banking_ledger_domain::currency::{
    CurrencySymbolChangeRejectionReason, CurrencySymbolChangeResult,
};
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

impl From<CurrencySymbolChangeResult> for CurrencySymbolChangeOutput {
    fn from(value: CurrencySymbolChangeResult) -> Self {
        match value {
            CurrencySymbolChangeResult::Changed => Self::Changed,
            CurrencySymbolChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
