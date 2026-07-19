use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// Describes a user organization invitation list user snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationInvitationListUserUpsert {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
