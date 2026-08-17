use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName,
    CurrencySymbol, MintAccountAddress,
};
use serde::{Deserialize, Serialize};

use super::{FragmentOwner, MaterializedCurrencyStatus};

/// Complete currency fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyFragment {
    pub id: CurrencyId,
    pub owner: FragmentOwner,
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
        self.owner
            .observations()
            .into_iter()
            .chain([self.observation])
            .collect()
    }
}

impl ReadModelFragment for CurrencyFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("currency_fragment");

    type Key = CurrencyId;

    fn key(&self) -> Self::Key {
        self.id
    }
}
