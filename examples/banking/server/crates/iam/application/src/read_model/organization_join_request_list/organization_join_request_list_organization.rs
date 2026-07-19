use appletheia::domain::EventId;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Organization profile owning an organization join request list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListOrganization {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}

impl OrganizationJoinRequestListOrganization {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        self.observation.event_ids().collect()
    }
}
