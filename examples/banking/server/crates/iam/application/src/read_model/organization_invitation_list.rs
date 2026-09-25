use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::{OrganizationFragment, OrganizationInvitationFragment, UserFragment};

mod organization_invitation_list_criteria;
mod organization_invitation_list_cursor;
mod organization_invitation_list_invitee;
mod organization_invitation_list_issuer;
mod organization_invitation_list_item;
mod organization_invitation_list_item_status;
mod organization_invitation_list_organization;
mod organization_invitation_list_reader;
mod organization_invitation_list_reader_error;
mod organization_invitation_list_sort_key;

pub use organization_invitation_list_criteria::OrganizationInvitationListCriteria;
pub use organization_invitation_list_cursor::OrganizationInvitationListCursor;
pub use organization_invitation_list_invitee::OrganizationInvitationListInvitee;
pub use organization_invitation_list_issuer::OrganizationInvitationListIssuer;
pub use organization_invitation_list_item::OrganizationInvitationListItem;
pub use organization_invitation_list_item_status::OrganizationInvitationListItemStatus;
pub use organization_invitation_list_organization::OrganizationInvitationListOrganization;
pub use organization_invitation_list_reader::OrganizationInvitationListReader;
pub use organization_invitation_list_reader_error::OrganizationInvitationListReaderError;
pub use organization_invitation_list_sort_key::OrganizationInvitationListSortKey;

/// Read model for organization-scoped invitation list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationInvitationList {
    pub organization: OrganizationInvitationListOrganization,
    pub items: Vec<OrganizationInvitationListItem>,
    pub start_cursor: Option<OrganizationInvitationListCursor>,
    pub end_cursor: Option<OrganizationInvitationListCursor>,
    pub has_previous: bool,
    pub has_next: bool,
}

impl ReadModelObservationSource for OrganizationInvitationList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.organization.observation)
            .chain(
                self.items
                    .iter()
                    .flat_map(|item| [item.observation, item.invitee.observation]),
            )
            .collect()
    }
}

impl ReadModel for OrganizationInvitationList {
    const NAME: ReadModelName = ReadModelName::new("organization_invitation_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = vec![SerializedPartition::try_from_fragment_key::<
            OrganizationFragment,
        >(&self.organization.organization_id)?];
        for item in &self.items {
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationInvitationFragment,
            >(&item.invitation_id)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<UserFragment>(
                &item.invitee.user_id,
            )?);
        }
        Ok(partitions)
    }
}
