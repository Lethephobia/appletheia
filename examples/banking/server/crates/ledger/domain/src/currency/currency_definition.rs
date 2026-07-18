use super::{
    CurrencyDecimals, CurrencyDescription, CurrencyImageRef, CurrencyName, CurrencyOwner,
    CurrencySymbol,
};

/// Describes a currency definition request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyDefinition {
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
    pub image: Option<CurrencyImageRef>,
}

impl CurrencyDefinition {
    pub(super) fn into_parts(
        self,
    ) -> (
        CurrencyOwner,
        CurrencySymbol,
        CurrencyName,
        CurrencyDecimals,
        Option<CurrencyDescription>,
        Option<CurrencyImageRef>,
    ) {
        (
            self.owner,
            self.symbol,
            self.name,
            self.decimals,
            self.description,
            self.image,
        )
    }
}
