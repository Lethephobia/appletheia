use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName,
    CurrencySymbol, MintAccountAddress,
};

use crate::projection::FragmentOwner;
use crate::read_model::CurrencyListItemOwner;

use super::{CurrencyFragment, MaterializedCurrencyStatus};

/// Read model for one public currency list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyListItemPart {
    pub currency_id: CurrencyId,
    pub owner: CurrencyListItemOwner,
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

impl From<CurrencyFragment> for CurrencyListItemPart {
    fn from(fragment: CurrencyFragment) -> Self {
        let owner = match fragment.owner {
            FragmentOwner::User(user) => CurrencyListItemOwner::User((*user).into()),
            FragmentOwner::Organization(organization) => {
                CurrencyListItemOwner::Organization((*organization).into())
            }
        };

        Self {
            currency_id: fragment.id,
            owner,
            symbol: fragment.symbol,
            name: fragment.name,
            decimals: fragment.decimals,
            description: fragment.description,
            image: fragment.image,
            mint_account_address: fragment.mint_account_address,
            supply: fragment.supply,
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for CurrencyListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.observation)
            .chain(self.owner.observations())
            .collect()
    }
}
