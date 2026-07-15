use banking_ledger_domain::currency_issuance::CurrencyIssuanceFailRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after failing a currency issuance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyIssuanceFailOutput {
    Failed,
    Rejected {
        reason: CurrencyIssuanceFailRejectionReason,
    },
}
