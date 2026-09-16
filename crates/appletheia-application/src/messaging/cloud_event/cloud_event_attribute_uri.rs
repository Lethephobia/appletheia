use std::{fmt, fmt::Display, str::FromStr};

use iri_string::{spec::UriSpec, validate::absolute_iri};
use serde::{Deserialize, Serialize};

use super::CloudEventAttributeUriError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventAttributeUri(String);

impl CloudEventAttributeUri {
    pub fn new(value: String) -> Result<Self, CloudEventAttributeUriError> {
        absolute_iri::<UriSpec>(&value).map_err(|_| CloudEventAttributeUriError::InvalidUri)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventAttributeUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventAttributeUri {
    type Err = CloudEventAttributeUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventAttributeUri {
    type Error = CloudEventAttributeUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventAttributeUri> for String {
    fn from(value: CloudEventAttributeUri) -> Self {
        value.0
    }
}
