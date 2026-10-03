use std::fmt::{self, Display};

use serde::Serialize;

use super::ReadModelResourceKeyError;

/// A JSON:API field name, excluding the reserved fields `id` and `type`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct ReadModelResourceKey(String);

impl ReadModelResourceKey {
    pub fn new(value: String) -> Result<Self, ReadModelResourceKeyError> {
        if value == "id" || value == "type" {
            return Err(ReadModelResourceKeyError::ReservedName(value));
        }
        let bytes = value.as_bytes();
        if bytes.is_empty()
            || !bytes.iter().enumerate().all(|(index, byte)| {
                byte.is_ascii_alphanumeric()
                    || *byte >= 0x80
                    || (matches!(byte, b'_' | b'-' | b' ') && index > 0 && index + 1 < bytes.len())
            })
        {
            return Err(ReadModelResourceKeyError::InvalidName(value));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
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
    fn validates_json_api_field_names() {
        for name in [
            "name",
            "display_name",
            "display-name",
            "display name",
            "名前",
            "a",
        ] {
            let key = ReadModelResourceKey::new(name.to_owned()).unwrap();
            assert_eq!(key.value(), name);
        }
        for name in [
            "", "_name", "name-", " name", "name ", "a.b", "a:b", "a/b", "a\n",
        ] {
            assert!(matches!(
                ReadModelResourceKey::new(name.to_owned()),
                Err(ReadModelResourceKeyError::InvalidName(_))
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
