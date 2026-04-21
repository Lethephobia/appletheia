use serde::{Deserialize, Serialize};

/// Returned after a currency removal request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRemoveOutput;
