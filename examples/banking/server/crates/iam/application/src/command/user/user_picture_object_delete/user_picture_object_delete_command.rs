use appletheia::command;
use banking_iam_domain::UserPictureObjectName;
use serde::{Deserialize, Serialize};

/// Deletes a user's picture object.
#[command(name = "user_picture_object_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureObjectDeleteCommand {
    pub object_name: UserPictureObjectName,
}
