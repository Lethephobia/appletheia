use serde::{Deserialize, Serialize};

use super::OwnedAccountClosureCountError;

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct OwnedAccountClosureCount(u32);

impl OwnedAccountClosureCount {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub fn try_increment(self) -> Result<Self, OwnedAccountClosureCountError> {
        self.try_add(1)
    }

    pub fn try_add(self, count: usize) -> Result<Self, OwnedAccountClosureCountError> {
        let increment =
            u32::try_from(count).map_err(|_| OwnedAccountClosureCountError::Overflow)?;
        self.0
            .checked_add(increment)
            .map(Self)
            .ok_or(OwnedAccountClosureCountError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_returns_a_new_count() {
        let count = OwnedAccountClosureCount::default();
        assert_eq!(count.try_increment().unwrap().value(), 1);
        assert_eq!(count.value(), 0);
        assert_eq!(
            OwnedAccountClosureCount::new(u32::MAX - 1)
                .try_increment()
                .unwrap()
                .value(),
            u32::MAX
        );
    }

    #[test]
    fn arithmetic_rejects_overflow() {
        let maximum = OwnedAccountClosureCount::new(u32::MAX);
        assert!(matches!(
            maximum.try_increment(),
            Err(OwnedAccountClosureCountError::Overflow)
        ));
        assert!(matches!(
            maximum.try_add(1),
            Err(OwnedAccountClosureCountError::Overflow)
        ));
        assert_eq!(maximum.try_add(0).unwrap(), maximum);
    }

    #[test]
    fn adds_multiple_items_without_changing_the_original_count() {
        let count = OwnedAccountClosureCount::new(2);
        assert_eq!(count.try_add(3).unwrap().value(), 5);
        assert_eq!(count.value(), 2);
        assert!(count.try_add(u32::MAX as usize).is_err());
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn rejects_increments_larger_than_u32() {
        assert!(matches!(
            OwnedAccountClosureCount::default().try_add(u32::MAX as usize + 1),
            Err(OwnedAccountClosureCountError::Overflow)
        ));
    }

    #[test]
    fn serialization_preserves_numeric_counts() {
        let count = OwnedAccountClosureCount::new(42);
        assert_eq!(serde_json::to_value(count).unwrap(), serde_json::json!(42));
        assert_eq!(
            serde_json::from_value::<OwnedAccountClosureCount>(serde_json::json!(42)).unwrap(),
            count
        );
        assert!(serde_json::from_value::<OwnedAccountClosureCount>(serde_json::json!(-1)).is_err());
    }
}
