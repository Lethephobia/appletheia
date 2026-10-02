use std::fmt::{self, Display};

use serde::Serialize;

/// A static relationship key following JSON:API member-name rules.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct ReadModelRelationshipKey(&'static str);

impl ReadModelRelationshipKey {
    /// Creates a name usable in associated constants; panics for invalid names.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        assert!(!bytes.is_empty(), "relationship key must not be empty");
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            let globally_allowed = byte.is_ascii_alphanumeric() || byte >= 0x80;
            let separator = byte == b'_' || byte == b'-' || byte == b' ';
            assert!(
                globally_allowed || (separator && index > 0 && index + 1 < bytes.len()),
                "invalid relationship key"
            );
            index += 1;
        }
        assert!(
            !(bytes.len() == 2 && bytes[0] == b'i' && bytes[1] == b'd'),
            "id is a reserved field name"
        );
        assert!(
            !(bytes.len() == 4
                && bytes[0] == b't'
                && bytes[1] == b'y'
                && bytes[2] == b'p'
                && bytes[3] == b'e'),
            "type is a reserved field name"
        );
        Self(value)
    }

    pub const fn value(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for ReadModelRelationshipKey {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Display for ReadModelRelationshipKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
