use serde::{Deserialize, Serialize};

/// Returned after a currency-definition deactivation request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionDeactivateOutput;
