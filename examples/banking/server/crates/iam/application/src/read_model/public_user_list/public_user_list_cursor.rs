use appletheia::domain::EventOccurredAt;
use banking_iam_domain::UserId;

/// Cursor for public user list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicUserListCursor {
    pub created_at: EventOccurredAt,
    pub user_id: UserId,
}
