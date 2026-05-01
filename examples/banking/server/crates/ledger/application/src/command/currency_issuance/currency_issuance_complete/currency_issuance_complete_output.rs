use banking_ledger_domain::currency_issuance::{
    CurrencyIssuanceCompleteRejectionReason, CurrencyIssuanceCompleteResult,
};
use serde::{Deserialize, Serialize};

/// Returned after completing a currency issuance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyIssuanceCompleteOutput {
    Completed,
    Rejected {
        reason: CurrencyIssuanceCompleteRejectionReason,
    },
}

impl From<CurrencyIssuanceCompleteResult> for CurrencyIssuanceCompleteOutput {
    fn from(value: CurrencyIssuanceCompleteResult) -> Self {
        match value {
            CurrencyIssuanceCompleteResult::Completed => Self::Completed,
            CurrencyIssuanceCompleteResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
