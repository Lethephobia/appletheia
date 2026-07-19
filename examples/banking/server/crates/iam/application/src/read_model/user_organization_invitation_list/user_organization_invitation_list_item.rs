use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

use super::{
    UserOrganizationInvitationListIssuer, UserOrganizationInvitationListItemStatus,
    UserOrganizationInvitationListOrganization,
};

/// One user organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationInvitationListItem {
    pub invitation_id: OrganizationInvitationId,
    pub organization: UserOrganizationInvitationListOrganization,
    pub roles: OrganizationRoles,
    pub issuer: UserOrganizationInvitationListIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: UserOrganizationInvitationListItemStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl UserOrganizationInvitationListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.organization.observation.event_ids()),
        )
    }
}
