use serde::Serialize;

use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// User profile owning a user organization join request list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationJoinRequestListUser {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
