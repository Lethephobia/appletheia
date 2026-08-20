use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{OrganizationMembershipId, OrganizationRoles};

use super::UserOrganizationMembershipListOrganization;

/// One user organization membership list row.
///
/// `organization_membership_id` is the aggregate identifier that membership
/// commands address, so a caller can act on the row it just read.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationMembershipListItem {
    pub organization_membership_id: OrganizationMembershipId,
    pub organization: UserOrganizationMembershipListOrganization,
    pub roles: OrganizationRoles,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl UserOrganizationMembershipListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.organization.observation.event_ids()),
        )
    }
}
