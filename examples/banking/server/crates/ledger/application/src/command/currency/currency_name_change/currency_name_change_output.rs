use banking_ledger_domain::currency::{
    CurrencyNameChangeRejectionReason, CurrencyNameChangeResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyNameChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyNameChangeRejectionReason,
    },
}

impl From<CurrencyNameChangeResult> for CurrencyNameChangeOutput {
    fn from(value: CurrencyNameChangeResult) -> Self {
        match value {
            CurrencyNameChangeResult::Changed => Self::Changed,
            CurrencyNameChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
