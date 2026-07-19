use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

use super::PublicUserListItemStatus;

/// Describes a public user list item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicUserListUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub status: PublicUserListItemStatus,
}
