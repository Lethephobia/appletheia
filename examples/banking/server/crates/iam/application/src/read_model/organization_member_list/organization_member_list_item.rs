use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::OrganizationRoles;

use super::OrganizationMemberListMember;

/// One organization member list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationMemberListItem {
    pub member: OrganizationMemberListMember,
    pub roles: OrganizationRoles,
    pub is_owner: bool,
    pub joined_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OrganizationMemberListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.member.observation.event_ids()),
        )
    }
}
