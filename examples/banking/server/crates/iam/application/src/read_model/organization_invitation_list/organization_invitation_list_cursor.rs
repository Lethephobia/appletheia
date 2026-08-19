use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationInvitationId;

/// Cursor for organization invitation list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct OrganizationInvitationListCursor {
    pub created_at: EventOccurredAt,
    pub invitation_id: OrganizationInvitationId,
}
