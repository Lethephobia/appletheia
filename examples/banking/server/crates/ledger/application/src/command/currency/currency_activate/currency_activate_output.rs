use banking_ledger_domain::currency::{CurrencyActivateRejectionReason, CurrencyActivateResult};
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

impl From<CurrencyActivateResult> for CurrencyActivateOutput {
    fn from(value: CurrencyActivateResult) -> Self {
        match value {
            CurrencyActivateResult::Activated => Self::Activated,
            CurrencyActivateResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
