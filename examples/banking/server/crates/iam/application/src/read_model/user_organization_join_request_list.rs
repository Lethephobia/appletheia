use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};

use crate::projection::{
    InternalUserSummaryPart, OrganizationFragment, OrganizationJoinRequestFragment, UserFragment,
    UserOrganizationJoinRequestListItemPart,
};

mod user_organization_join_request_list_criteria;
mod user_organization_join_request_list_cursor;
mod user_organization_join_request_list_reader;
mod user_organization_join_request_list_reader_error;
mod user_organization_join_request_list_sort_key;

pub use user_organization_join_request_list_criteria::UserOrganizationJoinRequestListCriteria;
pub use user_organization_join_request_list_cursor::UserOrganizationJoinRequestListCursor;
pub use user_organization_join_request_list_reader::UserOrganizationJoinRequestListReader;
pub use user_organization_join_request_list_reader_error::UserOrganizationJoinRequestListReaderError;
pub use user_organization_join_request_list_sort_key::UserOrganizationJoinRequestListSortKey;

/// Read model for user-scoped organization join request list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestList {
    pub user: InternalUserSummaryPart,
    pub items: Vec<UserOrganizationJoinRequestListItemPart>,
    pub next_cursor: Option<UserOrganizationJoinRequestListCursor>,
}

impl ReadModelObservationSource for UserOrganizationJoinRequestList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.user
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

impl ReadModel for UserOrganizationJoinRequestList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_join_request_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            OrganizationJoinRequestFragment,
        >(map_join_request_to_user_list)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field::<InternalUserSummaryPart>(
                "user",
                read_model.map(|read_model| &read_model.user),
            ),
            ReadModelPartTree::collection_with_explicit_route::<
                UserOrganizationJoinRequestListItemPart,
            >(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}
fn map_join_request_to_user_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<
        OrganizationJoinRequestFragment,
        UserOrganizationJoinRequestListItemPart,
    >(
        change,
        path_resolver,
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                UserFragment,
            >(&fragment.requester.id)?])
        },
        |_| Ok(Vec::new()),
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&fragment.organization.id)?])
        },
    )
}
