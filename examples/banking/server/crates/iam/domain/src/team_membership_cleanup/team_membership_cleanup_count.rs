use serde::{Deserialize, Serialize};

use super::TeamMembershipCleanupCountError;

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct TeamMembershipCleanupCount(u32);

impl TeamMembershipCleanupCount {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub fn try_increment(self) -> Result<Self, TeamMembershipCleanupCountError> {
        self.try_add(1)
    }

    pub fn try_add(self, count: usize) -> Result<Self, TeamMembershipCleanupCountError> {
        let increment =
            u32::try_from(count).map_err(|_| TeamMembershipCleanupCountError::Overflow)?;
        self.0
            .checked_add(increment)
            .map(Self)
            .ok_or(TeamMembershipCleanupCountError::Overflow)
    }
}
