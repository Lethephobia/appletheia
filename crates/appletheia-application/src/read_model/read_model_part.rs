use serde::{Serialize, de::DeserializeOwned};

use super::{
    ReadModelFragment, ReadModelObservationSource, ReadModelPartName, ReadModelPartTree,
    ReadModelPartition,
};

/// Defines one complete, replaceable part in a read model's client-facing change protocol.
pub trait ReadModelPart:
    ReadModelObservationSource
    + From<Self::SourceFragment>
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
    + Sized
    + 'static
{
    /// Identifies this part within its owning read model protocol.
    const NAME: ReadModelPartName;

    /// Declares the physical fragment that constructs and identifies this delivery part.
    type SourceFragment: ReadModelFragment;

    /// Returns this part's source-fragment key.
    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key;

    /// Returns this part's source-fragment partition for replacement or removal.
    fn partition(&self) -> ReadModelPartition<<Self::SourceFragment as ReadModelFragment>::Key> {
        ReadModelPartition::new(self.key())
    }

    /// Returns child parts with different source fragments and their attributes.
    ///
    /// Passing `None` declares the type-level child tree. Passing a part value attaches the
    /// corresponding materialized child values. Data derived from this part's own source fragment
    /// belongs directly in this part instead of another nested part.
    fn parts(_part: Option<&Self>) -> Vec<ReadModelPartTree> {
        Vec::new()
    }
}
