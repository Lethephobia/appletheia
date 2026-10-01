use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Read model for one public user list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicUserListItem {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub created_at: EventOccurredAt,
}
