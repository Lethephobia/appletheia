use appletheia::command;
use banking_iam_domain::TeamMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_promote_to_manager")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipPromoteToManagerCommand {
    pub team_membership_id: TeamMembershipId,
}
