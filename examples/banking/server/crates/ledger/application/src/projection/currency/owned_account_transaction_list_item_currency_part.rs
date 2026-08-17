use serde::{Deserialize, Serialize};

use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol, MintAccountAddress,
};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};

use super::CurrencyFragment;

/// Currency snapshot for owned account transaction list rows.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedAccountTransactionListItemCurrencyPart {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub mint_account_address: Option<MintAccountAddress>,
    pub observation: ReadModelObservation,
}

impl From<CurrencyFragment> for OwnedAccountTransactionListItemCurrencyPart {
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

impl ReadModelObservationSource for OwnedAccountTransactionListItemCurrencyPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}
