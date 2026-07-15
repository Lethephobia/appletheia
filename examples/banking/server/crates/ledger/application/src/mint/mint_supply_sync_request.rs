use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintSupplySyncRequest {
    currency_id: CurrencyId,
    decimals: CurrencyDecimals,
    target_supply: CurrencyAmount,
}

impl MintSupplySyncRequest {
    pub fn new(
        currency_id: CurrencyId,
        decimals: CurrencyDecimals,
        target_supply: CurrencyAmount,
    ) -> Self {
        Self {
            currency_id,
            decimals,
            target_supply,
        }
    }

    pub fn currency_id(&self) -> CurrencyId {
        self.currency_id
    }

    pub fn decimals(&self) -> CurrencyDecimals {
        self.decimals
    }

    pub fn target_supply(&self) -> CurrencyAmount {
        self.target_supply
    }
}
