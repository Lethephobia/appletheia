use appletheia::application::object_storage::{
    ObjectChecksum, ObjectContentLength, ObjectContentType,
};
use appletheia::command;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Prepares a signed upload request for an organization's picture.
#[command(name = "organization_prepare_picture_upload")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPreparePictureUploadCommand {
    pub organization_id: OrganizationId,
    pub content_type: ObjectContentType,
    pub content_length: ObjectContentLength,
    pub checksum: ObjectChecksum,
}
