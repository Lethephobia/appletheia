use appletheia::command;
use banking_iam_domain::{OrganizationId, TeamDescription, TeamName};
use serde::{Deserialize, Serialize};

#[command(name = "team_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamCreateCommand {
    pub organization_id: OrganizationId,
    pub name: TeamName,
    pub description: Option<TeamDescription>,
}
