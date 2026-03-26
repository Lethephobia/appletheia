use std::fmt::{self, Display};

/// Represents a borrowed command name used as a stable identifier.
///
/// Command names must be non-empty snake_case ASCII strings containing only
/// `a-z`, `0-9`, and `_`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandName(&'static str);

impl CommandName {
    /// The maximum allowed length for a command name.
    pub const MAX_LENGTH: usize = 100;

    /// Creates a command name from a static string.
    ///
    /// This function panics when `value` is empty, too long, or not snake_case ASCII.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();
        if len == 0 {
            panic!("command name is empty");
        }
        if len > Self::MAX_LENGTH {
            panic!("command name is too long");
        }
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            let is_lower = b >= b'a' && b <= b'z';
            let is_digit = b >= b'0' && b <= b'9';
            let is_underscore = b == b'_';

            if !(is_lower || is_digit || is_underscore) {
                panic!("command name must be snake_case ascii: [a-z0-9_]");
            }

            i += 1;
        }
        Self(value)
    }

    /// Returns the underlying command name.
    pub fn value(&self) -> &'static str {
        self.0
    }
}

impl Display for CommandName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::CommandName;

    #[test]
    fn new_accepts_valid_snake_case_value() {
        let command_name = CommandName::new("user_logout");

        assert_eq!(command_name.value(), "user_logout");
    }

    #[test]
    #[should_panic(expected = "command name is empty")]
    fn new_rejects_empty_value() {
        let _ = CommandName::new("");
    }

    #[test]
    #[should_panic(expected = "command name is too long")]
    fn new_rejects_too_long_value() {
        let _ = CommandName::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        );
    }

    #[test]
    #[should_panic(expected = "command name must be snake_case ascii: [a-z0-9_]")]
    fn new_rejects_non_snake_case_value() {
        let _ = CommandName::new("UserLogout");
    }

    #[test]
    fn display_matches_inner_value() {
        let command_name = CommandName::new("user_logout");

        assert_eq!(command_name.to_string(), "user_logout");
    }
}
