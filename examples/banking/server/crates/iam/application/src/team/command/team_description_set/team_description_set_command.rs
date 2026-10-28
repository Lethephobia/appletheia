use appletheia::command;
use banking_iam_domain::{TeamDescription, TeamId};
use serde::{Deserialize, Serialize};

#[command(name = "team_description_set")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamDescriptionSetCommand {
    pub team_id: TeamId,
    pub description: Option<TeamDescription>,
}
