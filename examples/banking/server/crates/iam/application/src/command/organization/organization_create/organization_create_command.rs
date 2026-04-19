use appletheia::command;
use banking_iam_domain::{OrganizationHandle, OrganizationName, OrganizationOwner};
use serde::{Deserialize, Serialize};

/// Creates a new organization.
#[command(name = "organization_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationCreateCommand {
    pub owner: OrganizationOwner,
    pub handle: OrganizationHandle,
    pub name: OrganizationName,
}
