use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, UserStatus, Username};

/// Attributes required to upsert a normalized user view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserViewUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub status: UserStatus,
}
