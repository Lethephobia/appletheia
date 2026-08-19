use serde::Serialize;

use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol, MintAccountAddress,
};

use appletheia::application::read_model::ReadModelObservation;

/// Currency part of an owned account list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountListItemCurrency {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub mint_account_address: Option<MintAccountAddress>,
    pub observation: ReadModelObservation,
}
