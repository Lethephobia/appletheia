use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::{OrganizationFragment, OrganizationJoinRequestFragment, UserFragment};

mod user_organization_join_request_list_criteria;
mod user_organization_join_request_list_cursor;
mod user_organization_join_request_list_item;
mod user_organization_join_request_list_item_status;
mod user_organization_join_request_list_organization;
mod user_organization_join_request_list_reader;
mod user_organization_join_request_list_reader_error;
mod user_organization_join_request_list_sort_key;
mod user_organization_join_request_list_user;

pub use user_organization_join_request_list_criteria::UserOrganizationJoinRequestListCriteria;
pub use user_organization_join_request_list_cursor::UserOrganizationJoinRequestListCursor;
pub use user_organization_join_request_list_item::UserOrganizationJoinRequestListItem;
pub use user_organization_join_request_list_item_status::UserOrganizationJoinRequestListItemStatus;
pub use user_organization_join_request_list_organization::UserOrganizationJoinRequestListOrganization;
pub use user_organization_join_request_list_reader::UserOrganizationJoinRequestListReader;
pub use user_organization_join_request_list_reader_error::UserOrganizationJoinRequestListReaderError;
pub use user_organization_join_request_list_sort_key::UserOrganizationJoinRequestListSortKey;
pub use user_organization_join_request_list_user::UserOrganizationJoinRequestListUser;

/// Read model for user-scoped organization join request list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationJoinRequestList {
    pub user: UserOrganizationJoinRequestListUser,
    pub items: Vec<UserOrganizationJoinRequestListItem>,
    pub start_cursor: Option<UserOrganizationJoinRequestListCursor>,
    pub end_cursor: Option<UserOrganizationJoinRequestListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for UserOrganizationJoinRequestList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.user.observation)
            .chain(
                self.items
                    .iter()
                    .flat_map(|item| [item.observation, item.organization.observation]),
            )
            .collect()
    }
}

impl ReadModel for UserOrganizationJoinRequestList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_join_request_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = vec![SerializedPartition::try_from_fragment_key::<UserFragment>(
            &self.user.user_id,
        )?];
        for item in &self.items {
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationJoinRequestFragment,
            >(&item.join_request_id)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&item.organization.organization_id)?);
        }
        Ok(partitions)
    }
}
