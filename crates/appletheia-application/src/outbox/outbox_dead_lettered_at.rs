use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DeadLetteredAt(DateTime<Utc>);

impl DeadLetteredAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for DeadLetteredAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for DeadLetteredAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<DeadLetteredAt> for DateTime<Utc> {
    fn from(value: DeadLetteredAt) -> Self {
        value.0
    }
}

impl Display for DeadLetteredAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
