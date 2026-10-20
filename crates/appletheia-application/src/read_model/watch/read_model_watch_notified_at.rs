use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchNotifiedAt(DateTime<Utc>);

impl ReadModelWatchNotifiedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for ReadModelWatchNotifiedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for ReadModelWatchNotifiedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<ReadModelWatchNotifiedAt> for DateTime<Utc> {
    fn from(value: ReadModelWatchNotifiedAt) -> Self {
        value.value()
    }
}

impl Display for ReadModelWatchNotifiedAt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
