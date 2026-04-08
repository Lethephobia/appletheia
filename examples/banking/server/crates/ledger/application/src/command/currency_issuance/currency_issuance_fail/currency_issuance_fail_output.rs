use serde::{Deserialize, Serialize};

/// Returned after failing a currency issuance.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceFailOutput;
