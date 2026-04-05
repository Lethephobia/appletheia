use appletheia::command;
use banking_iam_domain::{OrganizationId, UserId};
use serde::{Deserialize, Serialize};

/// Creates an organization membership.
#[command(name = "organization_membership_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipCreateCommand {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
}
