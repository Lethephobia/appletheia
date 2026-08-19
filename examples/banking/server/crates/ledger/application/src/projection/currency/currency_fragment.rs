use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName,
    CurrencyOwner, CurrencySymbol, MintAccountAddress,
};
use serde::{Deserialize, Serialize};

use super::MaterializedCurrencyStatus;

/// Normalized currency fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyFragment {
    pub id: CurrencyId,
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
    pub image: Option<CurrencyImageRef>,
    pub mint_account_address: Option<MintAccountAddress>,
    pub supply: CurrencyAmount,
    pub status: MaterializedCurrencyStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for CurrencyFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for CurrencyFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("currency_fragment");

    type Key = CurrencyId;

    fn key(&self) -> Self::Key {
        self.id
    }
}
