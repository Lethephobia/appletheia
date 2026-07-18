use banking_ledger_domain::currency::{CurrencyDefineRejectionReason, CurrencyId};
use serde::{Deserialize, Serialize};

/// The output returned after defining a currency.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyDefineOutput {
    Defined {
        currency_id: CurrencyId,
    },
    Rejected {
        currency_id: CurrencyId,
        reason: CurrencyDefineRejectionReason,
    },
}
