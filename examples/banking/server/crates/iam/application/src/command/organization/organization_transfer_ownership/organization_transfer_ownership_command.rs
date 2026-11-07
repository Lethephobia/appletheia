use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationOwner};
use serde::{Deserialize, Serialize};

/// Transfers ownership of an organization.
#[command(name = "organization_transfer_ownership")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationTransferOwnershipCommand {
    pub organization_id: OrganizationId,
    pub owner: OrganizationOwner,
}
