use crate::{OrganizationId, UserId};

/// Describes an organization join request submission.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OrganizationJoinRequestSubmission {
    pub organization_id: OrganizationId,
    pub requester_id: UserId,
}

impl OrganizationJoinRequestSubmission {
    pub(super) fn into_parts(self) -> (OrganizationId, UserId) {
        (self.organization_id, self.requester_id)
    }
}
