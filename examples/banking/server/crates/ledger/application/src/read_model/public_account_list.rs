use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use banking_iam_application::{OrganizationFragment, UserFragment};

use crate::projection::{
    AccountFragment, CurrencyFragment, FragmentOwner, PublicAccountListItemCurrencyPart,
    PublicAccountListItemOwnerOrganizationPart, PublicAccountListItemOwnerUserPart,
    PublicAccountListItemPart,
};

mod public_account_list_criteria;
mod public_account_list_cursor;
mod public_account_list_item_owner;
mod public_account_list_reader;
mod public_account_list_reader_error;
mod public_account_list_sort_key;

pub use public_account_list_criteria::PublicAccountListCriteria;
pub use public_account_list_cursor::PublicAccountListCursor;
pub use public_account_list_item_owner::PublicAccountListItemOwner;
pub use public_account_list_reader::PublicAccountListReader;
pub use public_account_list_reader_error::PublicAccountListReaderError;
pub use public_account_list_sort_key::PublicAccountListSortKey;

/// Read model for public account list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountList {
    pub items: Vec<PublicAccountListItemPart>,
    pub next_cursor: Option<PublicAccountListCursor>,
}

impl ReadModelObservationSource for PublicAccountList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(ReadModelObservationSource::observations)
            .collect()
    }
}

impl ReadModel for PublicAccountList {
    const NAME: ReadModelName = ReadModelName::new("public_account_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = &[
        ReadModelPartChangeRoute::from_fragment::<AccountFragment>(map_account_to_public_list),
        ReadModelPartChangeRoute::from_fragment::<CurrencyFragment>(
            map_currency_to_public_account_list,
        ),
        ReadModelPartChangeRoute::from_fragment::<UserFragment>(map_user_to_public_account_owner),
        ReadModelPartChangeRoute::from_fragment::<OrganizationFragment>(
            map_organization_to_public_account_owner,
        ),
    ];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::collection_with_explicit_route::<
            PublicAccountListItemPart,
        >(
            "items",
            read_model.map(|read_model| read_model.items.as_slice()),
        )]
    }
}

fn map_account_to_public_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<AccountFragment, PublicAccountListItemPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |fragment| {
            Ok(vec![
                SerializedPartition::try_from_fragment_key::<CurrencyFragment>(
                    &fragment.currency.id,
                )?,
                fragment_owner_partition(&fragment.owner)?,
            ])
        },
    )
}

fn map_currency_to_public_account_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<CurrencyFragment, PublicAccountListItemCurrencyPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
}

fn map_user_to_public_account_owner(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<UserFragment, PublicAccountListItemOwnerUserPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
}

fn map_organization_to_public_account_owner(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<OrganizationFragment, PublicAccountListItemOwnerOrganizationPart>(
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
