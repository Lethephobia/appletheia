use serde::{Deserialize, Serialize};

/// Describes why a user-picture upload preparation was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserPictureUploadPrepareRejectionReason {
    UserInactive,
    UserRemoved,
    ContentLengthTooLarge,
    ContentTypeNotAllowed,
}
