use appletheia::domain::EventOccurredAt;
use banking_iam_domain::UserId;

/// Cursor for organization member list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationMemberListCursor {
    pub joined_at: EventOccurredAt,
    pub user_id: UserId,
}
