use std::{fmt, fmt::Display};

use super::EventSequenceError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventSequence(i64);

impl EventSequence {
    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }
}

impl TryFrom<i64> for EventSequence {
    type Error = EventSequenceError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(EventSequenceError::NegativeValue(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<EventSequence> for i64 {
    fn from(value: EventSequence) -> Self {
        value.0
    }
}

impl Display for EventSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_accepts_zero_and_positive_values() {
        let zero = EventSequence::try_from(0).expect("zero should be valid");
        assert_eq!(zero.value(), 0);

        let positive = EventSequence::try_from(42).expect("positive value should be valid");
        assert_eq!(positive.value(), 42);
    }

    #[test]
    fn try_from_rejects_negative_values() {
        let err = EventSequence::try_from(-1).expect_err("negative value should be rejected");
        match err {
            EventSequenceError::NegativeValue(v) => assert_eq!(v, -1),
        }
    }

    #[test]
    fn conversions_round_trip() {
        let seq = EventSequence::try_from(7).unwrap();
        let as_i64: i64 = seq.into();
        assert_eq!(as_i64, 7);
    }

    #[test]
    fn display_formats_value() {
        let seq = EventSequence::try_from(5).unwrap();
        assert_eq!(seq.to_string(), "5");
    }
}
