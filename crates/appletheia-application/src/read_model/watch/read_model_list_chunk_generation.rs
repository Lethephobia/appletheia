use serde::{Deserialize, Serialize};

/// Distinguishes successive registrations of one reloadable chunk.
#[derive(
    Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ReadModelListChunkGeneration(u64);

impl ReadModelListChunkGeneration {
    /// Returns the initial generation.
    pub const fn initial() -> Self {
        Self(0)
    }

    /// Returns the next generation, or `None` on overflow.
    pub fn checked_next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

    /// Returns the transport value.
    pub const fn value(self) -> u64 {
        self.0
    }
}
