use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_cleanup_start")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCleanupStartCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
