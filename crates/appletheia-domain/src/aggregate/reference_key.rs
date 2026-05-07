use std::fmt::{self, Display};

/// Identifies a reference-index category using a stable snake_case string.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ReferenceKey(&'static str);

impl ReferenceKey {
    pub const MAX_LENGTH: usize = 100;

    /// Creates a reference key from a snake_case ASCII string literal.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();
        if len == 0 {
            panic!("reference key is empty");
        }
        if len > Self::MAX_LENGTH {
            panic!("reference key is too long");
        }
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            let is_lower = b >= b'a' && b <= b'z';
            let is_digit = b >= b'0' && b <= b'9';
            let is_underscore = b == b'_';

            if !(is_lower || is_digit || is_underscore) {
                panic!("reference key must be snake_case ascii: [a-z0-9_]");
            }

            i += 1;
        }
        Self(value)
    }

    /// Returns the raw reference-key string.
    pub fn value(self) -> &'static str {
        self.0
    }
}

impl Display for ReferenceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
