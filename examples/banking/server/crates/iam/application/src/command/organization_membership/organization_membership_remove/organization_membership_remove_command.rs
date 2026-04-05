use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

/// Removes an organization membership.
#[command(name = "organization_membership_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRemoveCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
