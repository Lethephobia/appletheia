use banking_ledger_domain::currency_issuance::{
    CurrencyIssuanceId, CurrencyIssuanceIssueRejectionReason,
};
use serde::{Deserialize, Serialize};

/// The output returned after starting a currency issuance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyIssueOutput {
    Issued {
        currency_issuance_id: CurrencyIssuanceId,
    },
    Rejected {
        currency_issuance_id: CurrencyIssuanceId,
        reason: CurrencyIssuanceIssueRejectionReason,
    },
}
