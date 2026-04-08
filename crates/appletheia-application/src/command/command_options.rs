use super::{CommandConsistency, CommandFailureReaction};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommandOptions {
    pub consistency: CommandConsistency,
    pub failure_reaction: CommandFailureReaction,
}
