use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::read_model::pagination::CursorWindow;

use super::{ReadModelListChunkGeneration, ReadModelListChunkId};

/// Describes one active list chunk that the server must rematerialize.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelListChunkDescriptor {
    pub chunk_id: ReadModelListChunkId,
    pub generation: ReadModelListChunkGeneration,
    pub window: CursorWindow<Value>,
}
