use serde::{Deserialize, Serialize};

use super::{ReadModelFragment, ReadModelFragmentNameOwned};

/// Identifies a prospective set of Fragment partitions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelDependencyTopic(String);

impl ReadModelDependencyTopic {
    /// Creates the correctness-first topic containing every partition of `F`.
    pub fn all<F>() -> Self
    where
        F: ReadModelFragment,
    {
        let fragment_name = ReadModelFragmentNameOwned::from(F::NAME);
        Self(format!("{}/all", fragment_name.value()))
    }

    /// Returns the transport value of this topic.
    pub fn value(&self) -> &str {
        &self.0
    }
}
