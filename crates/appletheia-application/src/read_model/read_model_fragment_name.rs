use std::fmt::{self, Display};

/// Identifies a stored read model fragment with a stable wire and storage name.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ReadModelFragmentName(&'static str);

impl ReadModelFragmentName {
    pub const MAX_LENGTH: usize = 100;

    /// Creates a statically validated snake_case fragment name.
    pub const fn new(value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let length = bytes.len();
        if length == 0 {
            panic!("read model fragment name is empty");
        }
        if length > Self::MAX_LENGTH {
            panic!("read model fragment name is too long");
        }

        let mut index = 0;
        while index < length {
            let byte = bytes[index];
            let is_lowercase = byte >= b'a' && byte <= b'z';
            let is_digit = byte >= b'0' && byte <= b'9';
            let is_underscore = byte == b'_';

            if !(is_lowercase || is_digit || is_underscore) {
                panic!("read model fragment name must be snake_case ascii: [a-z0-9_]");
            }

            index += 1;
        }

        Self(value)
    }

    /// Returns the fragment name.
    pub fn value(&self) -> &'static str {
        self.0
    }
}

impl Display for ReadModelFragmentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}
