use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use banking_iam_application::{OrganizationFragment, UserFragment};

use crate::projection::{
    AccountFragment, CurrencyFragment, FragmentOwner, OwnedAccountListItemCurrencyPart,
    OwnedAccountListItemPart, OwnedAccountListOwnerOrganizationPart, OwnedAccountListOwnerUserPart,
};

mod owned_account_list_criteria;
mod owned_account_list_cursor;
mod owned_account_list_owner;
mod owned_account_list_reader;
mod owned_account_list_reader_error;
mod owned_account_list_sort_key;

pub use owned_account_list_criteria::OwnedAccountListCriteria;
pub use owned_account_list_cursor::OwnedAccountListCursor;
pub use owned_account_list_owner::OwnedAccountListOwner;
pub use owned_account_list_reader::OwnedAccountListReader;
pub use owned_account_list_reader_error::OwnedAccountListReaderError;
pub use owned_account_list_sort_key::OwnedAccountListSortKey;

/// Read model for account list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountList {
    pub owner: OwnedAccountListOwner,
    pub items: Vec<OwnedAccountListItemPart>,
    pub next_cursor: Option<OwnedAccountListCursor>,
}

impl ReadModelObservationSource for OwnedAccountList {
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

impl ReadModel for OwnedAccountList {
    const NAME: ReadModelName = ReadModelName::new("owned_account_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = &[
        ReadModelPartChangeRoute::from_fragment::<UserFragment>(map_user_to_owned_account_owner),
        ReadModelPartChangeRoute::from_fragment::<OrganizationFragment>(
            map_organization_to_owned_account_owner,
        ),
        ReadModelPartChangeRoute::from_fragment::<AccountFragment>(map_account_to_owned_list),
        ReadModelPartChangeRoute::from_fragment::<CurrencyFragment>(
            map_currency_to_owned_account_list,
        ),
    ];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        let owner = read_model.map(|read_model| &read_model.owner);
        let owner_user = owner.and_then(|owner| match owner {
            OwnedAccountListOwner::User(user) => Some(user),
            OwnedAccountListOwner::Organization(_) => None,
        });
        let owner_organization = owner.and_then(|owner| match owner {
            OwnedAccountListOwner::User(_) => None,
            OwnedAccountListOwner::Organization(organization) => Some(organization),
        });

        vec![
            ReadModelPartTree::field_with_explicit_route::<OwnedAccountListOwnerUserPart>(
                "owner", owner_user,
            ),
            ReadModelPartTree::field_with_explicit_route::<OwnedAccountListOwnerOrganizationPart>(
                "owner",
                owner_organization,
            ),
            ReadModelPartTree::collection_with_explicit_route::<OwnedAccountListItemPart>(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}

fn map_account_to_owned_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<AccountFragment, OwnedAccountListItemPart>(
        change,
        path_resolver,
        |fragment| fragment_owner_partition(&fragment.owner).map(|partition| vec![partition]),
        |_| Ok(Vec::new()),
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                CurrencyFragment,
            >(&fragment.currency.id)?])
        },
    )
}

fn map_currency_to_owned_account_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<CurrencyFragment, OwnedAccountListItemCurrencyPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
}

fn map_user_to_owned_account_owner(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<UserFragment, OwnedAccountListOwnerUserPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
}

fn map_organization_to_owned_account_owner(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<OrganizationFragment, OwnedAccountListOwnerOrganizationPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
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
