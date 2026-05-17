use banking_ledger_domain::currency::{
    CurrencyImageChangeRejectionReason, CurrencyImageChangeResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after changing a currency image.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyImageChangeOutput {
    Changed,
    Rejected {
        reason: CurrencyImageChangeRejectionReason,
    },
}

impl From<CurrencyImageChangeResult> for CurrencyImageChangeOutput {
    fn from(value: CurrencyImageChangeResult) -> Self {
        match value {
            CurrencyImageChangeResult::Changed => Self::Changed,
            CurrencyImageChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
