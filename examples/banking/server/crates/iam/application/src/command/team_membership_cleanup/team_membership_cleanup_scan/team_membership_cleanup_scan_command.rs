use appletheia::command;
use banking_iam_domain::team_membership_cleanup::TeamMembershipCleanupId;
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_cleanup_scan")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCleanupScanCommand {
    pub team_membership_cleanup_id: TeamMembershipCleanupId,
}
