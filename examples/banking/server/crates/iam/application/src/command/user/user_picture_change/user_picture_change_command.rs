use appletheia::command;
use banking_iam_domain::{UserId, UserPictureRef};
use serde::{Deserialize, Serialize};

/// Changes a user's picture.
#[command(name = "user_picture_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureChangeCommand {
    pub user_id: UserId,
    pub picture: Option<UserPictureRef>,
}
