use serde::{Deserialize, Serialize};

/// The output returned after updating a currency definition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDefinitionUpdateOutput;
