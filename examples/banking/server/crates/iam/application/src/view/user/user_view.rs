use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, UserStatus, Username};

/// Represents the normalized user view persisted by read projections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserView {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub status: UserStatus,
}
