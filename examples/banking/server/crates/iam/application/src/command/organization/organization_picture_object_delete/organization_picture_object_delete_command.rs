use appletheia::command;
use banking_iam_domain::OrganizationPictureObjectName;
use serde::{Deserialize, Serialize};

/// Deletes an organization's picture object.
#[command(name = "organization_picture_object_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureObjectDeleteCommand {
    pub object_name: OrganizationPictureObjectName,
}
