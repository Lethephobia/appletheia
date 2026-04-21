use serde::{Deserialize, Serialize};

/// Returned after increasing currency supply.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIncreaseSupplyOutput;
