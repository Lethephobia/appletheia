use appletheia::command;
use banking_iam_domain::OrganizationPictureObjectName;
use serde::{Deserialize, Serialize};

/// Deletes an organization's picture object.
#[command(name = "organization_delete_picture_object")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDeletePictureObjectCommand {
    pub object_name: OrganizationPictureObjectName,
}
