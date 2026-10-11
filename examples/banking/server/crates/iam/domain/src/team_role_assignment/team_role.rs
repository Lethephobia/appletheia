use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TeamRole {
    Manager,
}

impl TeamRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manager => "manager",
        }
    }
}
