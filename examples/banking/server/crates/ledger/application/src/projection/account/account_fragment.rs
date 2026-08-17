use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountName};
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

use super::{CurrencyFragment, FragmentOwner, MaterializedAccountStatus};

/// Complete account fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountFragment {
    pub id: AccountId,
    pub owner: FragmentOwner,
    pub name: AccountName,
    pub currency: CurrencyFragment,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: MaterializedAccountStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for AccountFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.owner
            .observations()
            .into_iter()
            .chain(self.currency.observations())
            .chain([self.observation])
            .collect()
    }
}

impl ReadModelFragment for AccountFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("account_fragment");

    type Key = AccountId;

    fn key(&self) -> Self::Key {
        self.id
    }
}
