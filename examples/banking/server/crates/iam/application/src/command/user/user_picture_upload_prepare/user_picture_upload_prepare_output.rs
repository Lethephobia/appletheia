use appletheia::application::object_storage::SignedObjectUpload;
use banking_iam_domain::UserPictureRef;
use serde::{Deserialize, Serialize};

/// The output returned after preparing a user-picture upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureUploadPrepareOutput {
    pub picture: UserPictureRef,
    pub signed_upload: SignedObjectUpload,
}

impl UserPictureUploadPrepareOutput {
    /// Creates a new user-picture-upload-prepare output.
    pub fn new(picture: UserPictureRef, signed_upload: SignedObjectUpload) -> Self {
        Self {
            picture,
            signed_upload,
        }
    }
}
