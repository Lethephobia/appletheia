use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationRole, UserId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Creates an organization membership for a user.
#[command(name = "organization_membership_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipCreateCommand {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub roles: BTreeSet<OrganizationRole>,
}
