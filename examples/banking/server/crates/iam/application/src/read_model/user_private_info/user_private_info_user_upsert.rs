use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::UserPrivateInfoStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoUserUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub status: UserPrivateInfoStatus,
}
