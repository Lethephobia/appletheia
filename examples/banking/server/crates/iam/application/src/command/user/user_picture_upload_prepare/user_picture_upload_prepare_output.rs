use appletheia::application::object_storage::SignedObjectUpload;
use banking_iam_domain::UserPictureRef;
use serde::{Deserialize, Serialize};

use super::UserPictureUploadPrepareRejectionReason;

/// The output returned after preparing a user-picture upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserPictureUploadPrepareOutput {
    Prepared {
        picture: UserPictureRef,
        signed_upload: Box<SignedObjectUpload>,
    },
    Rejected {
        reason: UserPictureUploadPrepareRejectionReason,
    },
}
