use std::borrow::Borrow;
use std::fmt::{self, Display};

/// Identifies a relation definition using a stable snake_case string literal.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelationName(&'static str);

impl RelationName {
    pub const MAX_LENGTH: usize = 50;

    /// Creates a relation name from a validated snake_case string literal.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();
        if len == 0 {
            panic!("relation name is empty");
        }
        if len > Self::MAX_LENGTH {
            panic!("relation name is too long");
        }

        let mut i = 0;
        while i < len {
            let b = bytes[i];
            let is_lower = b >= b'a' && b <= b'z';
            let is_digit = b >= b'0' && b <= b'9';
            let is_underscore = b == b'_';

            if !(is_lower || is_digit || is_underscore) {
                panic!("relation name must be snake_case ascii: [a-z0-9_]");
            }

            i += 1;
        }

        Self(value)
    }

    /// Returns the raw relation name.
    pub fn value(self) -> &'static str {
        self.0
    }
}

impl Display for RelationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl AsRef<str> for RelationName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Borrow<str> for RelationName {
    fn borrow(&self) -> &str {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::RelationName;

    #[test]
    fn new_accepts_valid_snake_case_value() {
        let relation = RelationName::new("viewer");

        assert_eq!(relation.value(), "viewer");
    }

    #[test]
    #[should_panic(expected = "relation name is empty")]
    fn new_rejects_empty_value() {
        let _ = RelationName::new("");
    }

    #[test]
    #[should_panic(expected = "relation name is too long")]
    fn new_rejects_too_long_value() {
        let _ = RelationName::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }

    #[test]
    #[should_panic(expected = "relation name must be snake_case ascii: [a-z0-9_]")]
    fn new_rejects_non_snake_case_value() {
        let _ = RelationName::new("Viewer");
    }

    #[test]
    fn display_matches_inner_value() {
        let relation = RelationName::new("owner");

        assert_eq!(relation.to_string(), "owner");
    }
}
