use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol, MintAccountAddress,
};

use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Currency fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListItemCurrency {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub mint_account_address: Option<MintAccountAddress>,
    pub observation: ReadModelObservation,
}
