use appletheia::command;
use banking_iam_domain::{TeamId, TeamName};
use serde::{Deserialize, Serialize};

#[command(name = "team_name_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamNameChangeCommand {
    pub team_id: TeamId,
    pub name: TeamName,
}
