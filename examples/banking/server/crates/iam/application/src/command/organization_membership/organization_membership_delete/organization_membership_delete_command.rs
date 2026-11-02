use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

/// Deletes an organization membership.
///
/// Removal is terminal; rejoining the organization creates a new membership.
#[command(name = "organization_membership_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipDeleteCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
