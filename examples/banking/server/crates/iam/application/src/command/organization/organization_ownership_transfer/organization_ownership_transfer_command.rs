use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationOwner};
use serde::{Deserialize, Serialize};

/// Transfers ownership of an organization.
#[command(name = "organization_ownership_transfer")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationOwnershipTransferCommand {
    pub organization_id: OrganizationId,
    pub owner: OrganizationOwner,
}
