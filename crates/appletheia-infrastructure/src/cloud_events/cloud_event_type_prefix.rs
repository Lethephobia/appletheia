use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventTypePrefixError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventTypePrefix(String);

impl CloudEventTypePrefix {
    pub fn new(value: String) -> Result<Self, CloudEventTypePrefixError> {
        if value.is_empty() {
            return Err(CloudEventTypePrefixError::Empty);
        }
        if value.split('.').any(str::is_empty) {
            return Err(CloudEventTypePrefixError::EmptySegment);
        }
        if value.chars().any(|character| {
            let code = character as u32;
            character.is_control()
                || (0xfdd0..=0xfdef).contains(&code)
                || code & 0xffff == 0xfffe
                || code & 0xffff == 0xffff
        }) {
            return Err(CloudEventTypePrefixError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventTypePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventTypePrefix {
    type Err = CloudEventTypePrefixError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventTypePrefix {
    type Error = CloudEventTypePrefixError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventTypePrefix> for String {
    fn from(value: CloudEventTypePrefix) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_prefix_segments_and_characters_during_construction_and_deserialization() {
        for invalid in [
            "",
            ".example",
            "example.",
            "example..events",
            "example\n",
            "example\u{fdd0}",
        ] {
            assert!(CloudEventTypePrefix::new(invalid.to_owned()).is_err());
            assert!(
                serde_json::from_value::<CloudEventTypePrefix>(serde_json::json!(invalid)).is_err()
            );
        }
        let prefix: CloudEventTypePrefix = "example.events".parse().unwrap();
        assert_eq!(
            serde_json::from_str::<CloudEventTypePrefix>(&serde_json::to_string(&prefix).unwrap())
                .unwrap(),
            prefix
        );
    }
}
