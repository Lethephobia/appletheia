use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventAttributeString;
use super::{CloudEventTypeError, CloudEventTypePrefix};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventType(String);

impl CloudEventType {
    pub fn new(value: String) -> Result<Self, CloudEventTypeError> {
        if value.is_empty() {
            return Err(CloudEventTypeError::Empty);
        }
        CloudEventAttributeString::new(value.clone())?;
        Ok(Self(value))
    }

    pub fn with_prefix(
        prefix: Option<&CloudEventTypePrefix>,
        name: &str,
    ) -> Result<Self, CloudEventTypeError> {
        if name.is_empty() {
            return Err(CloudEventTypeError::Empty);
        }
        Self::new(match prefix {
            Some(prefix) => format!("{prefix}.{name}"),
            None => name.to_owned(),
        })
    }

    pub fn without_prefix<'a>(
        &'a self,
        prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<&'a str, CloudEventTypeError> {
        match prefix {
            Some(prefix) => self
                .as_str()
                .strip_prefix(&format!("{prefix}."))
                .filter(|name| !name.is_empty())
                .ok_or(CloudEventTypeError::PrefixMismatch),
            None => Ok(self.as_str()),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventType {
    type Err = CloudEventTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventType {
    type Error = CloudEventTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventType> for String {
    fn from(value: CloudEventType) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_round_trip_checks_the_namespace_boundary() {
        let prefix = "example.command".parse().unwrap();
        let event_type = CloudEventType::with_prefix(Some(&prefix), "transfer.failed").unwrap();
        assert_eq!(event_type.as_str(), "example.command.transfer.failed");
        assert_eq!(
            event_type.without_prefix(Some(&prefix)).unwrap(),
            "transfer.failed"
        );
        assert_eq!(
            event_type.without_prefix(None).unwrap(),
            event_type.as_str()
        );
        assert!(
            event_type
                .without_prefix(Some(&"example.commands".parse().unwrap()))
                .is_err()
        );
        assert!(CloudEventType::with_prefix(Some(&prefix), "").is_err());
        assert!(
            CloudEventType::new("example.command.".to_owned())
                .unwrap()
                .without_prefix(Some(&prefix))
                .is_err()
        );
    }
}
