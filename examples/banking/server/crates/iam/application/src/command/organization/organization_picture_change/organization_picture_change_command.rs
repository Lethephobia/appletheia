use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationPictureRef};
use serde::{Deserialize, Serialize};

/// Changes an organization's picture.
#[command(name = "organization_picture_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureChangeCommand {
    pub organization_id: OrganizationId,
    pub picture: Option<OrganizationPictureRef>,
}
