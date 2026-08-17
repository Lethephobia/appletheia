use std::fmt::{self, Display};

/// Identifies one replaceable part in a read model's client-facing change protocol.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ReadModelPartName(&'static str);

impl ReadModelPartName {
    pub const MAX_LENGTH: usize = 100;

    /// Creates a statically validated snake_case part name.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let length = bytes.len();
        if length == 0 {
            panic!("read model part name is empty");
        }
        if length > Self::MAX_LENGTH {
            panic!("read model part name is too long");
        }

        let mut index = 0;
        while index < length {
            let byte = bytes[index];
            let is_lowercase = byte >= b'a' && byte <= b'z';
            let is_digit = byte >= b'0' && byte <= b'9';
            let is_underscore = byte == b'_';

            if !(is_lowercase || is_digit || is_underscore) {
                panic!("read model part name must be snake_case ascii: [a-z0-9_]");
            }

            index += 1;
        }

        Self(value)
    }

    /// Returns the part name.
    pub fn value(&self) -> &'static str {
        self.0
    }
}

impl Display for ReadModelPartName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}
