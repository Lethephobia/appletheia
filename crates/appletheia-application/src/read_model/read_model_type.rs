use std::fmt::{self, Display};

use serde::Serialize;

/// A static resource type following JSON:API member-name rules.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct ReadModelType(&'static str);

impl ReadModelType {
    /// Creates a name usable in associated constants; panics for invalid names.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        assert!(!bytes.is_empty(), "resource type must not be empty");
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            let globally_allowed = byte.is_ascii_alphanumeric() || byte >= 0x80;
            let separator = byte == b'_' || byte == b'-' || byte == b' ';
            assert!(
                globally_allowed || (separator && index > 0 && index + 1 < bytes.len()),
                "invalid resource type"
            );
            index += 1;
        }
        Self(value)
    }

    pub const fn value(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for ReadModelType {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Display for ReadModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
