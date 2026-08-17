use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use banking_iam_application::{OrganizationFragment, UserFragment};
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;

use crate::projection::{FragmentOwner, WalletBookmarkFragment, WalletBookmarkListItemPart};

mod wallet_bookmark_list_criteria;
mod wallet_bookmark_list_cursor;
mod wallet_bookmark_list_reader;
mod wallet_bookmark_list_reader_error;
mod wallet_bookmark_list_sort_key;

pub use wallet_bookmark_list_criteria::WalletBookmarkListCriteria;
pub use wallet_bookmark_list_cursor::WalletBookmarkListCursor;
pub use wallet_bookmark_list_reader::WalletBookmarkListReader;
pub use wallet_bookmark_list_reader_error::WalletBookmarkListReaderError;
pub use wallet_bookmark_list_sort_key::WalletBookmarkListSortKey;

/// Read model for wallet bookmark list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletBookmarkList {
    pub owner: WalletBookmarkOwner,
    pub items: Vec<WalletBookmarkListItemPart>,
    pub next_cursor: Option<WalletBookmarkListCursor>,
}

impl ReadModelObservationSource for WalletBookmarkList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(ReadModelObservationSource::observations)
            .collect()
    }
}

impl ReadModel for WalletBookmarkList {
    const NAME: ReadModelName = ReadModelName::new("wallet_bookmark_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            WalletBookmarkFragment,
        >(map_wallet_bookmark_to_list)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::collection_with_explicit_route::<
            WalletBookmarkListItemPart,
        >(
            "items",
            read_model.map(|read_model| read_model.items.as_slice()),
        )]
    }
}
fn map_wallet_bookmark_to_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<WalletBookmarkFragment, WalletBookmarkListItemPart>(
        change,
        path_resolver,
        |fragment| fragment_owner_partition(&fragment.owner).map(|partition| vec![partition]),
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
