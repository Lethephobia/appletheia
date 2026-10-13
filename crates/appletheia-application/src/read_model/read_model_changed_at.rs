use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelChangedAt(DateTime<Utc>);

impl ReadModelChangedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for ReadModelChangedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for ReadModelChangedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<ReadModelChangedAt> for DateTime<Utc> {
    fn from(value: ReadModelChangedAt) -> Self {
        value.value()
    }
}

impl Display for ReadModelChangedAt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
