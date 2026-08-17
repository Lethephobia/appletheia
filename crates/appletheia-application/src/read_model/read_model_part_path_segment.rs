use serde::{Deserialize, Serialize};

/// Identifies one stable step in a read model part replacement path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ReadModelPartPathSegment {
    Attribute(String),
    Key(serde_json::Value),
}
