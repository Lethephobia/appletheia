use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use banking_iam_application::{OrganizationFragment, UserFragment};

use crate::projection::{
    CurrencyFragment, CurrencyListItemOwnerOrganizationPart, CurrencyListItemOwnerUserPart,
    CurrencyListItemPart, FragmentOwner,
};

mod currency_list_criteria;
mod currency_list_cursor;
mod currency_list_item_owner;
mod currency_list_reader;
mod currency_list_reader_error;
mod currency_list_sort_key;

pub use currency_list_criteria::CurrencyListCriteria;
pub use currency_list_cursor::CurrencyListCursor;
pub use currency_list_item_owner::CurrencyListItemOwner;
pub use currency_list_reader::CurrencyListReader;
pub use currency_list_reader_error::CurrencyListReaderError;
pub use currency_list_sort_key::CurrencyListSortKey;

/// Read model for public currency list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyList {
    pub items: Vec<CurrencyListItemPart>,
    pub next_cursor: Option<CurrencyListCursor>,
}

impl ReadModelObservationSource for CurrencyList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(ReadModelObservationSource::observations)
            .collect()
    }
}

impl ReadModel for CurrencyList {
    const NAME: ReadModelName = ReadModelName::new("currency_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = &[
        ReadModelPartChangeRoute::from_fragment::<CurrencyFragment>(map_currency_to_currency_list),
        ReadModelPartChangeRoute::from_fragment::<UserFragment>(map_user_to_currency_owner),
        ReadModelPartChangeRoute::from_fragment::<OrganizationFragment>(
            map_organization_to_currency_owner,
        ),
    ];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::collection_with_explicit_route::<
            CurrencyListItemPart,
        >(
            "items",
            read_model.map(|read_model| read_model.items.as_slice()),
        )]
    }
}

fn map_currency_to_currency_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<CurrencyFragment, CurrencyListItemPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |fragment| fragment_owner_partition(&fragment.owner).map(|partition| vec![partition]),
    )
}

fn map_user_to_currency_owner(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<UserFragment, CurrencyListItemOwnerUserPart>(
        change,
        path_resolver,
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
        |_| Ok(Vec::new()),
    )
}

fn map_organization_to_currency_owner(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<OrganizationFragment, CurrencyListItemOwnerOrganizationPart>(
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
