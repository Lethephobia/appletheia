use appletheia::command;
use banking_iam_domain::{TeamId, TeamName};
use serde::{Deserialize, Serialize};

#[command(name = "team_change_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamChangeNameCommand {
    pub team_id: TeamId,
    pub name: TeamName,
}
