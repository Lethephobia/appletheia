use std::{fmt, fmt::Display, str::FromStr};

use iri_string::{spec::UriSpec, validate::iri_reference};
use serde::{Deserialize, Serialize};

use super::CloudEventAttributeUriReferenceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventAttributeUriReference(String);

impl CloudEventAttributeUriReference {
    pub fn new(value: String) -> Result<Self, CloudEventAttributeUriReferenceError> {
        iri_reference::<UriSpec>(&value)
            .map_err(|_| CloudEventAttributeUriReferenceError::InvalidUri)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventAttributeUriReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventAttributeUriReference {
    type Err = CloudEventAttributeUriReferenceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventAttributeUriReference {
    type Error = CloudEventAttributeUriReferenceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventAttributeUriReference> for String {
    fn from(value: CloudEventAttributeUriReference) -> Self {
        value.0
    }
}
