use appletheia::command;
use banking_iam_domain::TeamId;
use serde::{Deserialize, Serialize};

#[command(name = "team_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamDeleteCommand {
    pub team_id: TeamId,
}
