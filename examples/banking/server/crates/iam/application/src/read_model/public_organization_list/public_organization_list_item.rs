use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Read model for one public organization list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicOrganizationListItem {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl PublicOrganizationListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        self.observation.event_ids().collect()
    }
}
