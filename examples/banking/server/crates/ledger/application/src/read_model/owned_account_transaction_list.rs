use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use banking_iam_application::{OrganizationFragment, UserFragment};

use crate::projection::{
    AccountFragment, AccountTransactionFragment, CurrencyFragment, FragmentOwner,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
    OwnedAccountTransactionListItemCounterpartyAccountPart,
    OwnedAccountTransactionListItemCurrencyPart, OwnedAccountTransactionListItemPart,
    OwnedAccountTransactionListOwnerOrganizationPart, OwnedAccountTransactionListOwnerUserPart,
};

mod owned_account_transaction_list_criteria;
mod owned_account_transaction_list_cursor;
mod owned_account_transaction_list_item_counterparty_account_owner;
mod owned_account_transaction_list_item_kind;
mod owned_account_transaction_list_owner;
mod owned_account_transaction_list_reader;
mod owned_account_transaction_list_reader_error;
mod owned_account_transaction_list_sort_key;

pub use owned_account_transaction_list_criteria::OwnedAccountTransactionListCriteria;
pub use owned_account_transaction_list_cursor::OwnedAccountTransactionListCursor;
pub use owned_account_transaction_list_item_counterparty_account_owner::OwnedAccountTransactionListItemCounterpartyAccountOwner;
pub use owned_account_transaction_list_item_kind::OwnedAccountTransactionListItemKind;
pub use owned_account_transaction_list_owner::OwnedAccountTransactionListOwner;
pub use owned_account_transaction_list_reader::OwnedAccountTransactionListReader;
pub use owned_account_transaction_list_reader_error::OwnedAccountTransactionListReaderError;
pub use owned_account_transaction_list_sort_key::OwnedAccountTransactionListSortKey;

/// Read model for owned account transaction list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionList {
    pub owner: OwnedAccountTransactionListOwner,
    pub items: Vec<OwnedAccountTransactionListItemPart>,
    pub next_cursor: Option<OwnedAccountTransactionListCursor>,
}

impl ReadModelObservationSource for OwnedAccountTransactionList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.owner
            .observations()
            .into_iter()
            .chain(
                self.items
                    .iter()
                    .flat_map(ReadModelObservationSource::observations),
            )
            .collect()
    }
}

impl ReadModel for OwnedAccountTransactionList {
    const NAME: ReadModelName = ReadModelName::new("owned_account_transaction_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = &[
        ReadModelPartChangeRoute::from_fragment::<UserFragment>(map_user_to_transaction_owners),
        ReadModelPartChangeRoute::from_fragment::<OrganizationFragment>(
            map_organization_to_transaction_owners,
        ),
        ReadModelPartChangeRoute::from_fragment::<AccountTransactionFragment>(
            map_transaction_to_list,
        ),
        ReadModelPartChangeRoute::from_fragment::<CurrencyFragment>(
            map_currency_to_transaction_list,
        ),
        ReadModelPartChangeRoute::from_fragment::<AccountFragment>(
            map_account_to_transaction_counterparty,
        ),
    ];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        let owner = read_model.map(|read_model| &read_model.owner);
        let owner_user = owner.and_then(|owner| match owner {
            OwnedAccountTransactionListOwner::User(user) => Some(user),
            OwnedAccountTransactionListOwner::Organization(_) => None,
        });
        let owner_organization = owner.and_then(|owner| match owner {
            OwnedAccountTransactionListOwner::User(_) => None,
            OwnedAccountTransactionListOwner::Organization(organization) => Some(organization),
        });

        vec![
            ReadModelPartTree::field_with_explicit_route::<OwnedAccountTransactionListOwnerUserPart>(
                "owner", owner_user,
            ),
            ReadModelPartTree::field_with_explicit_route::<
                OwnedAccountTransactionListOwnerOrganizationPart,
            >("owner", owner_organization),
            ReadModelPartTree::collection_with_explicit_route::<OwnedAccountTransactionListItemPart>(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}

fn map_account_to_transaction_counterparty(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<
        AccountFragment,
        OwnedAccountTransactionListItemCounterpartyAccountPart,
    >(
        change,
        path_resolver.clone(),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |fragment| fragment_owner_partition(&fragment.owner).map(|partition| vec![partition]),
    )
}

fn map_currency_to_transaction_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<CurrencyFragment, OwnedAccountTransactionListItemCurrencyPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
}

fn map_transaction_to_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<AccountTransactionFragment, OwnedAccountTransactionListItemPart>(
        change,
        path_resolver,
        |fragment| {
            fragment_owner_partition(&fragment.account.owner).map(|partition| vec![partition])
        },
        |_| Ok(Vec::new()),
        transaction_references,
    )
}

fn map_user_to_transaction_owners(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    let mut changes =
        ReadModelPartChange::map_one::<UserFragment, OwnedAccountTransactionListOwnerUserPart>(
            change,
            path_resolver.clone(),
            |_| Ok(Vec::new()),
            |_| Ok(Vec::new()),
            |_| Ok(Vec::new()),
        )?;
    changes.extend(ReadModelPartChange::map_one::<
        UserFragment,
        OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
    >(
        change,
        path_resolver.clone(),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )?);

    Ok(changes)
}

fn map_organization_to_transaction_owners(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    let mut changes = ReadModelPartChange::map_one::<
        OrganizationFragment,
        OwnedAccountTransactionListOwnerOrganizationPart,
    >(
        change,
        path_resolver.clone(),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )?;
    changes.extend(ReadModelPartChange::map_one::<
        OrganizationFragment,
        OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
    >(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )?);

    Ok(changes)
}

fn fragment_owner_partition(
    owner: &FragmentOwner,
) -> Result<SerializedPartition, ReadModelPartChangeError> {
    match owner {
        FragmentOwner::User(user) => Ok(
            SerializedPartition::try_from_fragment_key::<UserFragment>(&user.id)?,
        ),
        FragmentOwner::Organization(organization) => {
            Ok(SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&organization.id)?)
        }
    }
}

fn transaction_references(
    fragment: &AccountTransactionFragment,
) -> Result<Vec<SerializedPartition>, ReadModelPartChangeError> {
    let mut partitions = vec![SerializedPartition::try_from_fragment_key::<
        CurrencyFragment,
    >(&fragment.account.currency.id)?];
    if let Some(counterparty_account) = &fragment.counterparty_account {
        partitions.push(
            SerializedPartition::try_from_fragment_key::<AccountFragment>(
                &counterparty_account.id,
            )?,
        );
    }

    Ok(partitions)
}
