use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// The output returned after starting a currency issuance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssueOutput {
    pub currency_issuance_id: CurrencyIssuanceId,
}

impl CurrencyIssueOutput {
    /// Creates a new currency-issue output.
    pub fn new(currency_issuance_id: CurrencyIssuanceId) -> Self {
        Self {
            currency_issuance_id,
        }
    }
}
