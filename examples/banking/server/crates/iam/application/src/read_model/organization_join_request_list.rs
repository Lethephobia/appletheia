use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::{OrganizationFragment, OrganizationJoinRequestFragment, UserFragment};

mod organization_join_request_list_criteria;
mod organization_join_request_list_cursor;
mod organization_join_request_list_item;
mod organization_join_request_list_item_status;
mod organization_join_request_list_organization;
mod organization_join_request_list_reader;
mod organization_join_request_list_reader_error;
mod organization_join_request_list_requester;
mod organization_join_request_list_sort_key;

pub use organization_join_request_list_criteria::OrganizationJoinRequestListCriteria;
pub use organization_join_request_list_cursor::OrganizationJoinRequestListCursor;
pub use organization_join_request_list_item::OrganizationJoinRequestListItem;
pub use organization_join_request_list_item_status::OrganizationJoinRequestListItemStatus;
pub use organization_join_request_list_organization::OrganizationJoinRequestListOrganization;
pub use organization_join_request_list_reader::OrganizationJoinRequestListReader;
pub use organization_join_request_list_reader_error::OrganizationJoinRequestListReaderError;
pub use organization_join_request_list_requester::OrganizationJoinRequestListRequester;
pub use organization_join_request_list_sort_key::OrganizationJoinRequestListSortKey;

/// Read model for organization-scoped join request list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationJoinRequestList {
    pub organization: OrganizationJoinRequestListOrganization,
    pub items: Vec<OrganizationJoinRequestListItem>,
    pub start_cursor: Option<OrganizationJoinRequestListCursor>,
    pub end_cursor: Option<OrganizationJoinRequestListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for OrganizationJoinRequestList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.organization.observation)
            .chain(
                self.items
                    .iter()
                    .flat_map(|item| [item.observation, item.requester.observation]),
            )
            .collect()
    }
}

impl ReadModel for OrganizationJoinRequestList {
    const NAME: ReadModelName = ReadModelName::new("organization_join_request_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = vec![SerializedPartition::try_from_fragment_key::<
            OrganizationFragment,
        >(&self.organization.organization_id)?];
        for item in &self.items {
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationJoinRequestFragment,
            >(&item.join_request_id)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<UserFragment>(
                &item.requester.user_id,
            )?);
        }
        Ok(partitions)
    }
}
