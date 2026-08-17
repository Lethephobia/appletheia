use serde::{Serialize, de::DeserializeOwned};

use super::{ReadModelFragmentName, ReadModelObservationSource, ReadModelPartition};

/// Defines one independently stored read model fragment.
pub trait ReadModelFragment:
    ReadModelObservationSource + Serialize + DeserializeOwned + Send + Sync + Sized + 'static
{
    /// Identifies the physical fragment shared by read models.
    const NAME: ReadModelFragmentName;

    /// Identifies one stored fragment value.
    type Key: Clone + Serialize + DeserializeOwned + Send + Sync + Sized + 'static;

    /// Returns this fragment's physical key.
    fn key(&self) -> Self::Key;

    /// Returns this fragment's transport-neutral partition.
    fn partition(&self) -> ReadModelPartition<Self::Key> {
        ReadModelPartition::new(self.key())
    }
}
