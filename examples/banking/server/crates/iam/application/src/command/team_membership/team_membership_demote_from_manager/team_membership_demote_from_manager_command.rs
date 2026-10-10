use appletheia::command;
use banking_iam_domain::TeamMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_demote_from_manager")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipDemoteFromManagerCommand {
    pub team_membership_id: TeamMembershipId,
}
