use appletheia::command;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationOwner,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use serde::{Deserialize, Serialize};

/// Creates a new organization.
#[command(name = "organization_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationCreateCommand {
    pub owner: OrganizationOwner,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
}
