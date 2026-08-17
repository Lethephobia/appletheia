use serde::{Deserialize, Serialize};

use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol, MintAccountAddress,
};

use appletheia::application::read_model::ReadModelObservation;

use super::CurrencyFragment;

/// Currency fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicAccountListItemCurrencyPart {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub mint_account_address: Option<MintAccountAddress>,
    pub observation: ReadModelObservation,
}

impl From<CurrencyFragment> for PublicAccountListItemCurrencyPart {
    fn from(fragment: CurrencyFragment) -> Self {
        Self {
            id: fragment.id,
            symbol: fragment.symbol,
            name: fragment.name,
            decimals: fragment.decimals,
            mint_account_address: fragment.mint_account_address,
            observation: fragment.observation,
        }
    }
}
