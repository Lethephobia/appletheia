use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ReadModelListChunkGeneration, ReadModelListChunkId};

/// Contains one complete list chunk and its bidirectional cursor boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SerializedReadModelListChunk {
    pub chunk_id: ReadModelListChunkId,
    pub generation: ReadModelListChunkGeneration,
    pub items: Vec<Value>,
    pub start_cursor: Option<Value>,
    pub end_cursor: Option<Value>,
    pub has_previous: bool,
    pub has_next: bool,
}
