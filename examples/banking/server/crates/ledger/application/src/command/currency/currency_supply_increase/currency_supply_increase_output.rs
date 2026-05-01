use banking_ledger_domain::currency::{
    CurrencySupplyIncreaseRejectionReason, CurrencySupplyIncreaseResult,
};
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

impl From<CurrencySupplyIncreaseResult> for CurrencySupplyIncreaseOutput {
    fn from(value: CurrencySupplyIncreaseResult) -> Self {
        match value {
            CurrencySupplyIncreaseResult::Increased => Self::Increased,
            CurrencySupplyIncreaseResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
