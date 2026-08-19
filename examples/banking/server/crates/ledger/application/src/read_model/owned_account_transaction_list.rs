use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use banking_iam_application::{OrganizationFragment, UserFragment};
use serde::Serialize;

use crate::projection::{AccountFragment, AccountTransactionFragment, CurrencyFragment};

mod owned_account_transaction_id;
mod owned_account_transaction_list_criteria;
mod owned_account_transaction_list_cursor;
mod owned_account_transaction_list_item;
mod owned_account_transaction_list_item_counterparty_account;
mod owned_account_transaction_list_item_counterparty_account_owner;
mod owned_account_transaction_list_item_counterparty_account_owner_organization;
mod owned_account_transaction_list_item_counterparty_account_owner_user;
mod owned_account_transaction_list_item_currency;
mod owned_account_transaction_list_item_direction;
mod owned_account_transaction_list_item_kind;
mod owned_account_transaction_list_item_status;
mod owned_account_transaction_list_owner;
mod owned_account_transaction_list_owner_organization;
mod owned_account_transaction_list_owner_user;
mod owned_account_transaction_list_reader;
mod owned_account_transaction_list_reader_error;
mod owned_account_transaction_list_sort_key;

pub use owned_account_transaction_id::OwnedAccountTransactionId;
pub use owned_account_transaction_list_criteria::OwnedAccountTransactionListCriteria;
pub use owned_account_transaction_list_cursor::OwnedAccountTransactionListCursor;
pub use owned_account_transaction_list_item::OwnedAccountTransactionListItem;
pub use owned_account_transaction_list_item_counterparty_account::OwnedAccountTransactionListItemCounterpartyAccount;
pub use owned_account_transaction_list_item_counterparty_account_owner::OwnedAccountTransactionListItemCounterpartyAccountOwner;
pub use owned_account_transaction_list_item_counterparty_account_owner_organization::OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization;
pub use owned_account_transaction_list_item_counterparty_account_owner_user::OwnedAccountTransactionListItemCounterpartyAccountOwnerUser;
pub use owned_account_transaction_list_item_currency::OwnedAccountTransactionListItemCurrency;
pub use owned_account_transaction_list_item_direction::OwnedAccountTransactionListItemDirection;
pub use owned_account_transaction_list_item_kind::OwnedAccountTransactionListItemKind;
pub use owned_account_transaction_list_item_status::OwnedAccountTransactionListItemStatus;
pub use owned_account_transaction_list_owner::OwnedAccountTransactionListOwner;
pub use owned_account_transaction_list_owner_organization::OwnedAccountTransactionListOwnerOrganization;
pub use owned_account_transaction_list_owner_user::OwnedAccountTransactionListOwnerUser;
pub use owned_account_transaction_list_reader::OwnedAccountTransactionListReader;
pub use owned_account_transaction_list_reader_error::OwnedAccountTransactionListReaderError;
pub use owned_account_transaction_list_sort_key::OwnedAccountTransactionListSortKey;

/// Read model for owned account transaction list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountTransactionList {
    pub owner: OwnedAccountTransactionListOwner,
    pub items: Vec<OwnedAccountTransactionListItem>,
    pub start_cursor: Option<OwnedAccountTransactionListCursor>,
    pub end_cursor: Option<OwnedAccountTransactionListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for OwnedAccountTransactionList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        let owner = match &self.owner {
            OwnedAccountTransactionListOwner::User(owner) => owner.observation,
            OwnedAccountTransactionListOwner::Organization(owner) => owner.observation,
        };
        let mut observations = vec![owner];
        for item in &self.items {
            observations.extend([item.observation, item.currency.observation]);
            if let OwnedAccountTransactionListItemKind::Transfer {
                counterparty_account,
                ..
            } = &item.kind
            {
                observations.push(counterparty_account.observation);
                observations.push(match &counterparty_account.owner {
                    OwnedAccountTransactionListItemCounterpartyAccountOwner::User(owner) => {
                        owner.observation
                    }
                    OwnedAccountTransactionListItemCounterpartyAccountOwner::Organization(
                        owner,
                    ) => owner.observation,
                });
            }
        }
        observations
    }
}

impl ReadModel for OwnedAccountTransactionList {
    const NAME: ReadModelName = ReadModelName::new("owned_account_transaction_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = Vec::with_capacity(1 + self.items.len() * 4);
        let owner_partition = match &self.owner {
            OwnedAccountTransactionListOwner::User(owner) => {
                SerializedPartition::try_from_fragment_key::<UserFragment>(&owner.id)?
            }
            OwnedAccountTransactionListOwner::Organization(owner) => {
                SerializedPartition::try_from_fragment_key::<OrganizationFragment>(&owner.id)?
            }
        };
        partitions.push(owner_partition);

        for item in &self.items {
            let transaction_id =
                crate::projection::AccountTransactionId::from(item.transaction_id.value());
            partitions.push(SerializedPartition::try_from_fragment_key::<
                AccountTransactionFragment,
            >(&transaction_id)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<
                CurrencyFragment,
            >(&item.currency.id)?);

            if let OwnedAccountTransactionListItemKind::Transfer {
                counterparty_account,
                ..
            } = &item.kind
            {
                partitions.push(
                    SerializedPartition::try_from_fragment_key::<AccountFragment>(
                        &counterparty_account.id,
                    )?,
                );
                let counterparty_owner_partition = match &counterparty_account.owner {
                    OwnedAccountTransactionListItemCounterpartyAccountOwner::User(owner) => {
                        SerializedPartition::try_from_fragment_key::<UserFragment>(&owner.id)?
                    }
                    OwnedAccountTransactionListItemCounterpartyAccountOwner::Organization(
                        owner,
                    ) => SerializedPartition::try_from_fragment_key::<OrganizationFragment>(
                        &owner.id,
                    )?,
                };
                partitions.push(counterparty_owner_partition);
            }
        }
        Ok(partitions)
    }
}
