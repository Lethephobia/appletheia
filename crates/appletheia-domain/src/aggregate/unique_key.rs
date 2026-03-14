use std::fmt::{self, Display};

/// Identifies a unique-key category using a stable snake_case string.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UniqueKey(&'static str);

impl UniqueKey {
    pub const MAX_LENGTH: usize = 100;

    /// Creates a unique key from a snake_case ASCII string literal.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();
        if len == 0 {
            panic!("unique key is empty");
        }
        if len > Self::MAX_LENGTH {
            panic!("unique key is too long");
        }
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            let is_lower = b >= b'a' && b <= b'z';
            let is_digit = b >= b'0' && b <= b'9';
            let is_underscore = b == b'_';

            if !(is_lower || is_digit || is_underscore) {
                panic!("unique key must be snake_case ascii: [a-z0-9_]");
            }

            i += 1;
        }
        Self(value)
    }

    /// Returns the raw unique-key string.
    pub fn value(self) -> &'static str {
        self.0
    }
}

impl Display for UniqueKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueKey;

    #[test]
    fn new_accepts_valid_snake_case_value() {
        let unique_key = UniqueKey::new("email");

        assert_eq!(unique_key.value(), "email");
    }

    #[test]
    #[should_panic(expected = "unique key is empty")]
    fn new_rejects_empty_value() {
        let _ = UniqueKey::new("");
    }

    #[test]
    #[should_panic(expected = "unique key is too long")]
    fn new_rejects_too_long_value() {
        let _ = UniqueKey::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        );
    }

    #[test]
    #[should_panic(expected = "unique key must be snake_case ascii: [a-z0-9_]")]
    fn new_rejects_non_snake_case_value() {
        let _ = UniqueKey::new("UserEmail");
    }

    #[test]
    fn display_matches_inner_value() {
        let unique_key = UniqueKey::new("email");

        assert_eq!(unique_key.to_string(), "email");
    }
}
