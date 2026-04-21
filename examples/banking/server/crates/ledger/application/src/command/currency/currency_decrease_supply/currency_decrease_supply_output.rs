use serde::{Deserialize, Serialize};

/// Returned after decreasing currency supply.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDecreaseSupplyOutput;
