use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::UserPublicProfileStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPublicProfileUserUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub status: UserPublicProfileStatus,
}
