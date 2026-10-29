use appletheia::command;
use banking_iam_domain::TeamMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipDeleteCommand {
    pub team_membership_id: TeamMembershipId,
}
