use serde::{Deserialize, Serialize};

/// Returned after a currency image object delete request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyImageObjectDeleteOutput;
