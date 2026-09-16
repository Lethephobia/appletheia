use std::{fmt, fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::CloudEventTimeError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventTime(DateTime<Utc>);

impl CloudEventTime {
    pub fn new(value: DateTime<Utc>) -> Result<Self, CloudEventTimeError> {
        DateTime::parse_from_rfc3339(&value.to_rfc3339())?;
        Ok(Self(value))
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Display for CloudEventTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_rfc3339())
    }
}

impl FromStr for CloudEventTime {
    type Err = CloudEventTimeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
    }
}

impl TryFrom<String> for CloudEventTime {
    type Error = CloudEventTimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<CloudEventTime> for String {
    fn from(value: CloudEventTime) -> Self {
        value.to_string()
    }
}
