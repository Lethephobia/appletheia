use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::EventId;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Organization profile owning an organization member list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationMemberListOrganization {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}

impl OrganizationMemberListOrganization {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        self.observation.event_ids().collect()
    }
}
