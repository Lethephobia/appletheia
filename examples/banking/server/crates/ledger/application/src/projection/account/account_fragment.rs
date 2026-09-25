use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountDescription, AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

use super::MaterializedAccountStatus;

/// Normalized account fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountFragment {
    pub id: AccountId,
    pub owner: AccountOwner,
    pub name: AccountName,
    pub description: Option<AccountDescription>,
    pub currency_id: CurrencyId,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: MaterializedAccountStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for AccountFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for AccountFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("account_fragment");

    type Key = AccountId;

    fn key(&self) -> Self::Key {
        self.id
    }
}
