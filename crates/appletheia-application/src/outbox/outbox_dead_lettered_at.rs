use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxDeadLetteredAt(DateTime<Utc>);

impl OutboxDeadLetteredAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for OutboxDeadLetteredAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OutboxDeadLetteredAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OutboxDeadLetteredAt> for DateTime<Utc> {
    fn from(value: OutboxDeadLetteredAt) -> Self {
        value.0
    }
}

impl Display for OutboxDeadLetteredAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
