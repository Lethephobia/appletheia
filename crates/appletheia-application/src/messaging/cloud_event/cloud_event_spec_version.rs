use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventSpecVersionError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CloudEventSpecVersion {
    #[serde(rename = "1.0")]
    V1_0,
}

impl Display for CloudEventSpecVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("1.0")
    }
}

impl FromStr for CloudEventSpecVersion {
    type Err = CloudEventSpecVersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "1.0" => Ok(Self::V1_0),
            _ => Err(CloudEventSpecVersionError::UnsupportedVersion),
        }
    }
}
