use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountName};
use banking_ledger_domain::core::CurrencyAmount;

use crate::projection::OwnedAccountListItemCurrencyPart;

use super::{AccountFragment, MaterializedAccountStatus};

/// Read model for one account list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedAccountListItemPart {
    pub account_id: AccountId,
    pub name: AccountName,
    pub currency: OwnedAccountListItemCurrencyPart,
    pub balance: CurrencyAmount,
    pub reserved_balance: CurrencyAmount,
    pub status: MaterializedAccountStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<AccountFragment> for OwnedAccountListItemPart {
    fn from(fragment: AccountFragment) -> Self {
        Self {
            account_id: fragment.id,
            name: fragment.name,
            currency: fragment.currency.into(),
            balance: fragment.balance,
            reserved_balance: fragment.reserved_balance,
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for OwnedAccountListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.currency.observation]
    }
}
