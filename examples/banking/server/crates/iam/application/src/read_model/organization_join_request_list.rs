use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};

use crate::projection::{
    InternalOrganizationSummaryPart, OrganizationFragment, OrganizationJoinRequestFragment,
    OrganizationJoinRequestListItemPart, UserFragment,
};

mod organization_join_request_list_criteria;
mod organization_join_request_list_cursor;
mod organization_join_request_list_reader;
mod organization_join_request_list_reader_error;
mod organization_join_request_list_sort_key;

pub use organization_join_request_list_criteria::OrganizationJoinRequestListCriteria;
pub use organization_join_request_list_cursor::OrganizationJoinRequestListCursor;
pub use organization_join_request_list_reader::OrganizationJoinRequestListReader;
pub use organization_join_request_list_reader_error::OrganizationJoinRequestListReaderError;
pub use organization_join_request_list_sort_key::OrganizationJoinRequestListSortKey;

/// Read model for organization-scoped join request list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestList {
    pub organization: InternalOrganizationSummaryPart,
    pub items: Vec<OrganizationJoinRequestListItemPart>,
    pub next_cursor: Option<OrganizationJoinRequestListCursor>,
}

impl ReadModelObservationSource for OrganizationJoinRequestList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.organization
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

impl ReadModel for OrganizationJoinRequestList {
    const NAME: ReadModelName = ReadModelName::new("organization_join_request_list");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            OrganizationJoinRequestFragment,
        >(map_join_request_to_organization_list)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field::<InternalOrganizationSummaryPart>(
                "organization",
                read_model.map(|read_model| &read_model.organization),
            ),
            ReadModelPartTree::collection_with_explicit_route::<OrganizationJoinRequestListItemPart>(
                "items",
                read_model.map(|read_model| read_model.items.as_slice()),
            ),
        ]
    }
}
fn map_join_request_to_organization_list(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<
        OrganizationJoinRequestFragment,
        OrganizationJoinRequestListItemPart,
    >(
        change,
        path_resolver,
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&fragment.organization.id)?])
        },
        |_| Ok(Vec::new()),
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                UserFragment,
            >(&fragment.requester.id)?])
        },
    )
}
