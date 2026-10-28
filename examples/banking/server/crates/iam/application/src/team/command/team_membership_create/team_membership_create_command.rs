use appletheia::command;
use banking_iam_domain::{OrganizationMembershipId, TeamId};
use serde::{Deserialize, Serialize};

#[command(name = "team_membership_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCreateCommand {
    pub team_id: TeamId,
    pub organization_membership_id: OrganizationMembershipId,
}
