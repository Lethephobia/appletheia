use appletheia::command;
use banking_iam_domain::team_membership::TeamMembershipId;
use banking_iam_domain::team_membership_cleanup::TeamMembershipCleanupId;
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_cleanup_membership_succeeded_record")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCleanupMembershipSucceededRecordCommand {
    pub team_membership_cleanup_id: TeamMembershipCleanupId,
    pub team_membership_id: TeamMembershipId,
}
