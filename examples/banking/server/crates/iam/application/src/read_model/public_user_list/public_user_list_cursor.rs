use appletheia::domain::EventOccurredAt;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Cursor for public user list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PublicUserListCursor {
    pub created_at: EventOccurredAt,
    pub user_id: UserId,
}
