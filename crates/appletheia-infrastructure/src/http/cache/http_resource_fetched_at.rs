use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpResourceFetchedAt(DateTime<Utc>);

impl HttpResourceFetchedAt {
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
