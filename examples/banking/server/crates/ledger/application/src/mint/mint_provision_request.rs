use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName,
    CurrencySymbol,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintProvisionRequest {
    currency_id: CurrencyId,
    decimals: CurrencyDecimals,
    name: CurrencyName,
    symbol: CurrencySymbol,
    description: Option<CurrencyDescription>,
    image: Option<CurrencyImageRef>,
}

impl MintProvisionRequest {
    pub fn new(
        currency_id: CurrencyId,
        decimals: CurrencyDecimals,
        name: CurrencyName,
        symbol: CurrencySymbol,
        description: Option<CurrencyDescription>,
        image: Option<CurrencyImageRef>,
    ) -> Self {
        Self {
            currency_id,
            decimals,
            name,
            symbol,
            description,
            image,
        }
    }

    pub fn currency_id(&self) -> CurrencyId {
        self.currency_id
    }

    pub fn decimals(&self) -> CurrencyDecimals {
        self.decimals
    }

    pub fn name(&self) -> &CurrencyName {
        &self.name
    }

    pub fn symbol(&self) -> &CurrencySymbol {
        &self.symbol
    }

    pub fn description(&self) -> Option<&CurrencyDescription> {
        self.description.as_ref()
    }

    pub fn image(&self) -> Option<&CurrencyImageRef> {
        self.image.as_ref()
    }
}
