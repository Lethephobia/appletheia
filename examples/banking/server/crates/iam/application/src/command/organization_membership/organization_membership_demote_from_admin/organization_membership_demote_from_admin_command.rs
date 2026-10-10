use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "organization_membership_demote_from_admin")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipDemoteFromAdminCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
