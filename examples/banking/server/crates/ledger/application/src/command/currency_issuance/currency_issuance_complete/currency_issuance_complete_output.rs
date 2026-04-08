use serde::{Deserialize, Serialize};

/// Returned after completing a currency issuance.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceCompleteOutput;
