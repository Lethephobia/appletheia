use serde::{Deserialize, Serialize};

use super::CommandConsistency;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommandOptions {
    pub consistency: CommandConsistency,
}
