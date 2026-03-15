use std::fmt::{self, Display};

/// Identifies an aggregate category using a stable snake_case string.
///
/// Aggregate types are intended for persisted and external-facing identifiers,
/// so values are restricted to ASCII snake_case and a bounded maximum length.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AggregateType(&'static str);

impl AggregateType {
    pub const MAX_LENGTH: usize = 100;

    /// Creates an aggregate type from a snake_case ASCII string literal.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();
        if len == 0 {
            panic!("aggregate type is empty");
        }
        if len > Self::MAX_LENGTH {
            panic!("aggregate type is too long");
        }
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            let is_lower = b >= b'a' && b <= b'z';
            let is_digit = b >= b'0' && b <= b'9';
            let is_underscore = b == b'_';

            if !(is_lower || is_digit || is_underscore) {
                panic!("aggregate type must be snake_case ascii: [a-z0-9_]");
            }

            i += 1;
        }
        Self(value)
    }

    /// Returns the raw aggregate type string.
    pub fn value(self) -> &'static str {
        self.0
    }
}

impl Display for AggregateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_valid_snake_case_value() {
        let aggregate_type = AggregateType::new("bank_account");

        assert_eq!(aggregate_type.value(), "bank_account");
    }

    #[test]
    #[should_panic(expected = "aggregate type is empty")]
    fn new_rejects_empty_value() {
        let _ = AggregateType::new("");
    }

    #[test]
    #[should_panic(expected = "aggregate type is too long")]
    fn new_rejects_too_long_value() {
        let _ = AggregateType::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        );
    }

    #[test]
    #[should_panic(expected = "aggregate type must be snake_case ascii: [a-z0-9_]")]
    fn new_rejects_non_snake_case_value() {
        let _ = AggregateType::new("BankAccount");
    }

    #[test]
    fn value_returns_inner_str() {
        let aggregate_type = AggregateType::new("counter");

        assert_eq!(aggregate_type.value(), "counter");
    }

    #[test]
    fn display_matches_inner_value() {
        let aggregate_type = AggregateType::new("counter");

        assert_eq!(aggregate_type.to_string(), "counter");
    }
}
