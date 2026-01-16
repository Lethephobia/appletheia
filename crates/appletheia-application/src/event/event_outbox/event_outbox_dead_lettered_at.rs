use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventOutboxDeadLetteredAt(DateTime<Utc>);

impl EventOutboxDeadLetteredAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for EventOutboxDeadLetteredAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for EventOutboxDeadLetteredAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<EventOutboxDeadLetteredAt> for DateTime<Utc> {
    fn from(value: EventOutboxDeadLetteredAt) -> Self {
        value.0
    }
}

impl Display for EventOutboxDeadLetteredAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
