use std::{fmt, fmt::Display, str::FromStr};

use iri_string::{spec::UriSpec, validate::iri_reference};
use serde::{Deserialize, Serialize};

use super::CloudEventSourceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventSource(String);

impl CloudEventSource {
    pub fn new(value: String) -> Result<Self, CloudEventSourceError> {
        if value.is_empty() {
            return Err(CloudEventSourceError::Empty);
        }
        iri_reference::<UriSpec>(&value).map_err(|_| CloudEventSourceError::InvalidUri)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventSource {
    type Err = CloudEventSourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventSource {
    type Error = CloudEventSourceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventSource> for String {
    fn from(value: CloudEventSource) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_uri_references_during_construction_and_deserialization() {
        for valid in [
            "urn:banking",
            "https://example.com/events",
            "/events",
            "../events",
        ] {
            assert_eq!(valid.parse::<CloudEventSource>().unwrap().as_str(), valid);
        }
        for invalid in [
            "",
            "%zz",
            "https://example.com/a b",
            "https://[invalid",
            "日本語",
        ] {
            assert!(invalid.parse::<CloudEventSource>().is_err());
            assert!(
                serde_json::from_value::<CloudEventSource>(serde_json::json!(invalid)).is_err()
            );
        }
    }
}
