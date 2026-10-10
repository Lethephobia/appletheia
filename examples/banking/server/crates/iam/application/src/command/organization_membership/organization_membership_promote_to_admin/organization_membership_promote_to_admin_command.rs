use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "organization_membership_promote_to_admin")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipPromoteToAdminCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
