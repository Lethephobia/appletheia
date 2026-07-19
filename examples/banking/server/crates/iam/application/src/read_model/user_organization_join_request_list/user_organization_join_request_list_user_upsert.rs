use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Describes a user organization join request list user snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationJoinRequestListUserUpsert {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
