use appletheia::application::object_storage::SignedObjectUploadRequest;
use banking_iam_domain::UserPictureRef;
use serde::{Deserialize, Serialize};

/// The output returned after preparing a user-picture upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureUploadPrepareOutput {
    pub picture: UserPictureRef,
    pub signed_upload_request: SignedObjectUploadRequest,
}

impl UserPictureUploadPrepareOutput {
    /// Creates a new user-picture-upload-prepare output.
    pub fn new(picture: UserPictureRef, signed_upload_request: SignedObjectUploadRequest) -> Self {
        Self {
            picture,
            signed_upload_request,
        }
    }
}
