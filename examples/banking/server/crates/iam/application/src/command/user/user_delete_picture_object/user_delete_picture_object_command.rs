use appletheia::command;
use banking_iam_domain::UserPictureObjectName;
use serde::{Deserialize, Serialize};

/// Deletes a user's picture object.
#[command(name = "user_delete_picture_object")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDeletePictureObjectCommand {
    pub object_name: UserPictureObjectName,
}
