use super::ReadModelListChunkDescriptor;

/// Describes the complete materialization currently retained for a subscription.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadModelWatchRefreshRequest {
    Snapshot,
    List {
        active_chunks: Vec<ReadModelListChunkDescriptor>,
    },
}
