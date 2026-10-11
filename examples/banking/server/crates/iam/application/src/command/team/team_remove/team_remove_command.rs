use appletheia::command;
use banking_iam_domain::TeamId;
use serde::{Deserialize, Serialize};

#[command(name = "team_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamRemoveCommand {
    pub team_id: TeamId,
}
