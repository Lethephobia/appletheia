use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};

use super::{
    OrganizationInvitationListInvitee, OrganizationInvitationListIssuer,
    OrganizationInvitationListItemStatus,
};

/// One organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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
