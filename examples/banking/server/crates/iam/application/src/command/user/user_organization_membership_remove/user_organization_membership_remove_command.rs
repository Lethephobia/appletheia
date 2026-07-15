use appletheia::command;
use banking_iam_domain::{OrganizationId, UserId};
use serde::{Deserialize, Serialize};

/// Removes an organization membership from a user.
#[command(name = "user_organization_membership_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOrganizationMembershipRemoveCommand {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
}
