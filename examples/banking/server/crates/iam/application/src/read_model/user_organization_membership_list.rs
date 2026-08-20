use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::{
    OrganizationFragment, OrganizationMembershipFragment, OrganizationMembershipFragmentKey,
    UserFragment,
};

mod user_organization_membership_list_cursor;
mod user_organization_membership_list_item;
mod user_organization_membership_list_organization;
mod user_organization_membership_list_reader;
mod user_organization_membership_list_reader_error;
mod user_organization_membership_list_sort_key;
mod user_organization_membership_list_user;

pub use user_organization_membership_list_cursor::UserOrganizationMembershipListCursor;
pub use user_organization_membership_list_item::UserOrganizationMembershipListItem;
pub use user_organization_membership_list_organization::UserOrganizationMembershipListOrganization;
pub use user_organization_membership_list_reader::UserOrganizationMembershipListReader;
pub use user_organization_membership_list_reader_error::UserOrganizationMembershipListReaderError;
pub use user_organization_membership_list_sort_key::UserOrganizationMembershipListSortKey;
pub use user_organization_membership_list_user::UserOrganizationMembershipListUser;

/// Read model listing the organizations a user belongs to.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationMembershipList {
    pub user: UserOrganizationMembershipListUser,
    pub items: Vec<UserOrganizationMembershipListItem>,
    pub start_cursor: Option<UserOrganizationMembershipListCursor>,
    pub end_cursor: Option<UserOrganizationMembershipListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for UserOrganizationMembershipList {
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

impl ReadModel for UserOrganizationMembershipList {
    const NAME: ReadModelName = ReadModelName::new("user_organization_membership_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = vec![SerializedPartition::try_from_fragment_key::<UserFragment>(
            &self.user.user_id,
        )?];
        for item in &self.items {
            let membership_key = OrganizationMembershipFragmentKey {
                user_id: self.user.user_id,
                organization_id: item.organization.organization_id,
            };
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationMembershipFragment,
            >(&membership_key)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&item.organization.organization_id)?);
        }
        Ok(partitions)
    }
}
