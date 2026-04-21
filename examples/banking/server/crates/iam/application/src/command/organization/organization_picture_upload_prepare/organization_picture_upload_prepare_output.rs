use appletheia::application::object_storage::SignedObjectUploadRequest;
use banking_iam_domain::OrganizationPictureRef;
use serde::{Deserialize, Serialize};

/// The output returned after preparing an organization-picture upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureUploadPrepareOutput {
    pub picture: OrganizationPictureRef,
    pub signed_upload_request: SignedObjectUploadRequest,
}

impl OrganizationPictureUploadPrepareOutput {
    /// Creates a new organization-picture-upload-prepare output.
    pub fn new(
        picture: OrganizationPictureRef,
        signed_upload_request: SignedObjectUploadRequest,
    ) -> Self {
        Self {
            picture,
            signed_upload_request,
        }
    }
}
