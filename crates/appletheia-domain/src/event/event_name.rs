use std::fmt::{self, Display};

/// Represents the stable name of an event payload.
///
/// Event names must be non-empty ASCII snake_case strings and fit within
/// `MAX_LENGTH`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventName(&'static str);

impl EventName {
    pub const MAX_LENGTH: usize = 100;

    /// Creates an event name from a validated static string.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();
        if len == 0 {
            panic!("event name is empty");
        }
        if len > Self::MAX_LENGTH {
            panic!("event name is too long");
        }
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            let is_lower = b >= b'a' && b <= b'z';
            let is_digit = b >= b'0' && b <= b'9';
            let is_underscore = b == b'_';

            if !(is_lower || is_digit || is_underscore) {
                panic!("event name must be snake_case ascii: [a-z0-9_]");
            }

            i += 1;
        }

        Self(value)
    }

    /// Returns the raw event name.
    pub fn value(self) -> &'static str {
        self.0
    }
}

impl Display for EventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::EventName;

    #[test]
    fn new_accepts_valid_snake_case_value() {
        let name = EventName::new("counter_incremented");

        assert_eq!(name.value(), "counter_incremented");
    }

    #[test]
    #[should_panic(expected = "event name is empty")]
    fn new_rejects_empty_value() {
        let _ = EventName::new("");
    }

    #[test]
    #[should_panic(expected = "event name is too long")]
    fn new_rejects_too_long_value() {
        let _ = EventName::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        );
    }

    #[test]
    #[should_panic(expected = "event name must be snake_case ascii: [a-z0-9_]")]
    fn new_rejects_non_snake_case_value() {
        let _ = EventName::new("CounterIncremented");
    }

    #[test]
    fn value_returns_inner_str() {
        let name = EventName::new("user_renamed");

        assert_eq!(name.value(), "user_renamed");
    }

    #[test]
    fn display_matches_inner_value() {
        let name = EventName::new("account_opened");

        assert_eq!(name.to_string(), "account_opened");
    }
}
