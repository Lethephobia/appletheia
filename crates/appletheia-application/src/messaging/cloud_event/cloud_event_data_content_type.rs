use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CloudEventAttributeString, CloudEventDataContentTypeError};

/// A nonempty content type with CloudEvents string validation.
/// Full media-type grammar validation is left to the producer.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventDataContentType(String);

impl CloudEventDataContentType {
    pub fn new(value: String) -> Result<Self, CloudEventDataContentTypeError> {
        if value.is_empty() {
            return Err(CloudEventDataContentTypeError::Empty);
        }
        CloudEventAttributeString::new(value.clone())?;
        Ok(Self(value))
    }

    pub fn json() -> Self {
        Self("application/json".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_json(&self) -> bool {
        let media_type = self.0.split(';').next().unwrap_or_default().trim();
        let Some((_, subtype)) = media_type.split_once('/') else {
            return false;
        };
        subtype.eq_ignore_ascii_case("json")
            || subtype
                .rsplit_once('+')
                .is_some_and(|(_, suffix)| suffix.eq_ignore_ascii_case("json"))
    }
}

impl Display for CloudEventDataContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventDataContentType {
    type Err = CloudEventDataContentTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventDataContentType {
    type Error = CloudEventDataContentTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventDataContentType> for String {
    fn from(value: CloudEventDataContentType) -> Self {
        value.0
    }
}
