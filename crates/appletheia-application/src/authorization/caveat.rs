use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::CaveatName;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Caveat {
    pub name: CaveatName,
    pub params: Value,
}

