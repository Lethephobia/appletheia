use serde::{Deserialize, Serialize};

use super::{SerializedReadModelListChunk, SerializedReadModelSnapshot};

/// Contains the complete value produced by one server-side query refresh.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelWatchRefreshValue {
    Snapshot(SerializedReadModelSnapshot),
    List(Vec<SerializedReadModelListChunk>),
}
