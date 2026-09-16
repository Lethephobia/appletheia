use std::{fmt, fmt::Display, str::FromStr};

use iri_string::{spec::UriSpec, validate::absolute_iri};
use serde::{Deserialize, Serialize};

use super::CloudEventDataSchemaError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventDataSchema(String);

impl CloudEventDataSchema {
    pub fn new(value: String) -> Result<Self, CloudEventDataSchemaError> {
        absolute_iri::<UriSpec>(&value).map_err(|_| CloudEventDataSchemaError::InvalidUri)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventDataSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventDataSchema {
    type Err = CloudEventDataSchemaError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventDataSchema {
    type Error = CloudEventDataSchemaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventDataSchema> for String {
    fn from(value: CloudEventDataSchema) -> Self {
        value.0
    }
}
