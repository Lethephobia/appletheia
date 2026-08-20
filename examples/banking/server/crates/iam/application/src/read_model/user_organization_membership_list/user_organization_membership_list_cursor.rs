use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationMembershipId;

/// Cursor for user organization membership list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct UserOrganizationMembershipListCursor {
    pub created_at: EventOccurredAt,
    pub organization_membership_id: OrganizationMembershipId,
}
