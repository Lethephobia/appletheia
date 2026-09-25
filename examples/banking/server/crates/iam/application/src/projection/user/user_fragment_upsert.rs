use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::MaterializedUserStatus;

/// Values used to create or restore a public user fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserFragmentUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub status: MaterializedUserStatus,
}
