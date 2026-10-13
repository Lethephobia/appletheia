use std::fmt::{self, Display};

use serde::Serialize;

use super::ReadModelResourceKeyError;

/// An ASCII snake_case resource field name, excluding the reserved fields `id` and `type`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct ReadModelResourceKey(String);

impl ReadModelResourceKey {
    pub const MAX_LENGTH: usize = 100;

    pub fn new(value: String) -> Result<Self, ReadModelResourceKeyError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ReadModelResourceKeyError> {
        if value.is_empty() {
            return Err(ReadModelResourceKeyError::Empty);
        }
        let len = value.len();
        if len > Self::MAX_LENGTH {
            return Err(ReadModelResourceKeyError::TooLong {
                len,
                max: Self::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(ReadModelResourceKeyError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        if value == "id" || value == "type" {
            return Err(ReadModelResourceKeyError::ReservedName(value.to_owned()));
        }
        Ok(())
    }
}

impl AsRef<str> for ReadModelResourceKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ReadModelResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_snake_case_keys_and_reserved_names() {
        for name in ["name", "display_name", "_name", "name__2", "123"] {
            let key = ReadModelResourceKey::new(name.to_owned()).unwrap();
            assert_eq!(key.value(), name);
        }
        assert!(ReadModelResourceKey::new("a".repeat(100)).is_ok());
        assert!(matches!(
            ReadModelResourceKey::new(String::new()),
            Err(ReadModelResourceKeyError::Empty)
        ));
        assert!(matches!(
            ReadModelResourceKey::new("a".repeat(101)),
            Err(ReadModelResourceKeyError::TooLong { len: 101, max: 100 })
        ));
        for name in [
            "Name",
            "display-name",
            "display name",
            "名前",
            "a.b",
            "a:b",
            "a/b",
            "a\n",
        ] {
            assert!(matches!(
                ReadModelResourceKey::new(name.to_owned()),
                Err(ReadModelResourceKeyError::InvalidFormat { .. })
            ));
        }
        for name in ["id", "type"] {
            assert!(matches!(
                ReadModelResourceKey::new(name.to_owned()),
                Err(ReadModelResourceKeyError::ReservedName(_))
            ));
        }
    }
}
