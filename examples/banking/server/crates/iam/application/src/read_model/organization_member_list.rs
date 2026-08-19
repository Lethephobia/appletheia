use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::{
    OrganizationFragment, OrganizationMembershipFragment, OrganizationMembershipFragmentKey,
    UserFragment,
};

mod organization_member_list_criteria;
mod organization_member_list_cursor;
mod organization_member_list_item;
mod organization_member_list_member;
mod organization_member_list_organization;
mod organization_member_list_reader;
mod organization_member_list_reader_error;
mod organization_member_list_sort_key;

pub use organization_member_list_criteria::OrganizationMemberListCriteria;
pub use organization_member_list_cursor::OrganizationMemberListCursor;
pub use organization_member_list_item::OrganizationMemberListItem;
pub use organization_member_list_member::OrganizationMemberListMember;
pub use organization_member_list_organization::OrganizationMemberListOrganization;
pub use organization_member_list_reader::OrganizationMemberListReader;
pub use organization_member_list_reader_error::OrganizationMemberListReaderError;
pub use organization_member_list_sort_key::OrganizationMemberListSortKey;

/// Read model for organization member list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationMemberList {
    pub organization: OrganizationMemberListOrganization,
    pub items: Vec<OrganizationMemberListItem>,
    pub start_cursor: Option<OrganizationMemberListCursor>,
    pub end_cursor: Option<OrganizationMemberListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for OrganizationMemberList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.organization.observation)
            .chain(
                self.items
                    .iter()
                    .flat_map(|item| [item.observation, item.member.observation]),
            )
            .collect()
    }
}

impl ReadModel for OrganizationMemberList {
    const NAME: ReadModelName = ReadModelName::new("organization_member_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = vec![SerializedPartition::try_from_fragment_key::<
            OrganizationFragment,
        >(&self.organization.organization_id)?];
        for item in &self.items {
            let membership_key = OrganizationMembershipFragmentKey {
                user_id: item.member.user_id,
                organization_id: self.organization.organization_id,
            };
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationMembershipFragment,
            >(&membership_key)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<UserFragment>(
                &item.member.user_id,
            )?);
        }
        Ok(partitions)
    }
}
