use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::OrganizationJoinRequestId;
use banking_shared_kernel_application::read_model::ReadModelObservation;

use super::{OrganizationJoinRequestListItemStatus, OrganizationJoinRequestListRequester};

/// One organization join request list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListItem {
    pub join_request_id: OrganizationJoinRequestId,
    pub requester: OrganizationJoinRequestListRequester,
    pub status: OrganizationJoinRequestListItemStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OrganizationJoinRequestListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.requester.observation.event_ids()),
        )
    }
}
