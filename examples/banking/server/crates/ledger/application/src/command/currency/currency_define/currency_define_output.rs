use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// The output returned after defining a currency.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefineOutput {
    pub currency_id: CurrencyId,
}

impl CurrencyDefineOutput {
    /// Creates a new currency-define output.
    pub fn new(currency_id: CurrencyId) -> Self {
        Self { currency_id }
    }
}
