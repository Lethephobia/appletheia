use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::OrganizationJoinRequestId;

use super::{
    UserOrganizationJoinRequestListItemStatus, UserOrganizationJoinRequestListOrganization,
};

/// One user organization join request list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationJoinRequestListItem {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization: UserOrganizationJoinRequestListOrganization,
    pub status: UserOrganizationJoinRequestListItemStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl UserOrganizationJoinRequestListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.organization.observation.event_ids()),
        )
    }
}
