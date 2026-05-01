use banking_ledger_domain::currency::{
    CurrencyNameChangeRejectionReason, CurrencySymbolChangeRejectionReason,
};
use serde::{Deserialize, Serialize};

/// The output returned after updating a currency.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyUpdateOutput {
    Updated,
    Rejected {
        reason: CurrencyUpdateRejectionReason,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyUpdateRejectionReason {
    Removed,
}

impl From<CurrencySymbolChangeRejectionReason> for CurrencyUpdateRejectionReason {
    fn from(value: CurrencySymbolChangeRejectionReason) -> Self {
        match value {
            CurrencySymbolChangeRejectionReason::Removed => Self::Removed,
        }
    }
}

impl From<CurrencyNameChangeRejectionReason> for CurrencyUpdateRejectionReason {
    fn from(value: CurrencyNameChangeRejectionReason) -> Self {
        match value {
            CurrencyNameChangeRejectionReason::Removed => Self::Removed,
        }
    }
}
