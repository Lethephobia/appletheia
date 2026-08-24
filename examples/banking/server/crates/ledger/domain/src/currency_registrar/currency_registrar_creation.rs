use super::{CurrencyRegistrarDescription, CurrencyRegistrarDisplayName, CurrencyRegistrarHandle};
use serde::{Deserialize, Serialize};

/// Describes a CurrencyRegistrar creation request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRegistrarCreation {
    pub handle: CurrencyRegistrarHandle,
    pub display_name: CurrencyRegistrarDisplayName,
    pub description: Option<CurrencyRegistrarDescription>,
}

impl CurrencyRegistrarCreation {
    pub(super) fn into_parts(
        self,
    ) -> (
        CurrencyRegistrarHandle,
        CurrencyRegistrarDisplayName,
        Option<CurrencyRegistrarDescription>,
    ) {
        (self.handle, self.display_name, self.description)
    }
}
