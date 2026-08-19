use serde::{Deserialize, Serialize};

/// Orders complete snapshots delivered for one subscription.
#[derive(
    Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ReadModelWatchRevision(u64);

impl ReadModelWatchRevision {
    /// Returns the state before any snapshot has been delivered.
    pub const fn initial() -> Self {
        Self(0)
    }

    /// Returns the next revision, or `None` on overflow.
    pub fn checked_next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

    /// Returns the transport value.
    pub const fn value(self) -> u64 {
        self.0
    }
}
