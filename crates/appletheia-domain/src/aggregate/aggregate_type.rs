use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AggregateType(&'static str);

impl AggregateType {
    pub const MAX_LENGTH: usize = 100;

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

    pub fn value(self) -> &'static str {
        self.0
    }
}

impl Display for AggregateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
