use banking_ledger_domain::currency::{
    CurrencyDescriptionChangeRejectionReason, CurrencyDescriptionChangeResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency description.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyDescriptionChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyDescriptionChangeRejectionReason,
    },
}

impl From<CurrencyDescriptionChangeResult> for CurrencyDescriptionChangeOutput {
    fn from(value: CurrencyDescriptionChangeResult) -> Self {
        match value {
            CurrencyDescriptionChangeResult::Changed => Self::Changed,
            CurrencyDescriptionChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
