use banking_ledger_domain::currency::{
    CurrencySupplyDecreaseRejectionReason, CurrencySupplyDecreaseResult,
};
use serde::{Deserialize, Serialize};

/// Returned after decreasing currency supply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySupplyDecreaseOutput {
    Decreased,
    Rejected {
        reason: CurrencySupplyDecreaseRejectionReason,
    },
}

impl From<CurrencySupplyDecreaseResult> for CurrencySupplyDecreaseOutput {
    fn from(value: CurrencySupplyDecreaseResult) -> Self {
        match value {
            CurrencySupplyDecreaseResult::Decreased => Self::Decreased,
            CurrencySupplyDecreaseResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
