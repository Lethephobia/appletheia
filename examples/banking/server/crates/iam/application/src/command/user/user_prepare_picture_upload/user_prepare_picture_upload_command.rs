use appletheia::application::object_storage::{
    ObjectChecksum, ObjectContentLength, ObjectContentType,
};
use appletheia::command;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Prepares a signed upload request for a user's picture.
#[command(name = "user_prepare_picture_upload")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPreparePictureUploadCommand {
    pub user_id: UserId,
    pub content_type: ObjectContentType,
    pub content_length: ObjectContentLength,
    pub checksum: ObjectChecksum,
}
