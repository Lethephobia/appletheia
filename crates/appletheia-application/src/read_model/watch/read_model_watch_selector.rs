use serde::{Deserialize, Serialize};

use crate::read_model::{ReadModelFragment, ReadModelFragmentNameOwned, SerializedPartition};

/// Selects physical partitions or every partition of a Fragment for a watch subscription.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelWatchSelector {
    Partition(SerializedPartition),
    Fragment(ReadModelFragmentNameOwned),
}

impl ReadModelWatchSelector {
    pub fn fragment<F: ReadModelFragment>() -> Self {
        Self::Fragment(ReadModelFragmentNameOwned::from(F::NAME))
    }
}
