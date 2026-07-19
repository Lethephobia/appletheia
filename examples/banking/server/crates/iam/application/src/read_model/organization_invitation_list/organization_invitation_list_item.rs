use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

use super::{
    OrganizationInvitationListInvitee, OrganizationInvitationListIssuer,
    OrganizationInvitationListItemStatus,
};

/// One organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationListItem {
    pub invitation_id: OrganizationInvitationId,
    pub invitee: OrganizationInvitationListInvitee,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationListIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationListItemStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OrganizationInvitationListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.invitee.observation.event_ids()),
        )
    }
}
