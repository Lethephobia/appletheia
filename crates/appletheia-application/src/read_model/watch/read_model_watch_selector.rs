use serde::{Deserialize, Serialize};

use crate::read_model::{ReadModelFragment, ReadModelFragmentNameOwned, SerializedPartition};

/// Selects one partition or every partition of a fragment.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ReadModelWatchSelector {
    Partition(SerializedPartition),
    Fragment(ReadModelFragmentNameOwned),
}

impl ReadModelWatchSelector {
    pub fn fragment<F>() -> Self
    where
        F: ReadModelFragment,
    {
        Self::Fragment(F::NAME.into())
    }
}
