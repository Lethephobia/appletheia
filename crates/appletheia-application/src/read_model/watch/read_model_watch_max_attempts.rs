use std::num::NonZeroU32;

/// Limits refresh attempts for one watch update, including the initial attempt.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchMaxAttempts(NonZeroU32);

impl ReadModelWatchMaxAttempts {
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> NonZeroU32 {
        self.0
    }
}

impl Default for ReadModelWatchMaxAttempts {
    fn default() -> Self {
        Self(NonZeroU32::MIN.saturating_add(2))
    }
}
