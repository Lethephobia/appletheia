use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationInvitationId;

/// Cursor for user organization invitation list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserOrganizationInvitationListCursor {
    pub created_at: EventOccurredAt,
    pub invitation_id: OrganizationInvitationId,
}
