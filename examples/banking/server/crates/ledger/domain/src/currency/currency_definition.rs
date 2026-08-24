use serde::{Deserialize, Serialize};

use crate::core::{CurrencyCode, CurrencyDecimals};
use crate::currency_registrar::CurrencyRegistrarId;

use super::CurrencyDescription;

/// Describes a Currency definition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyDefinition {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub code: CurrencyCode,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
}
