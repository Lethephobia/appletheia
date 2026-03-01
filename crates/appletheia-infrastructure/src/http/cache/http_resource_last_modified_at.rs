use chrono::{DateTime, Utc};
use httpdate::{fmt_http_date, parse_http_date};
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpResourceLastModifiedAt(DateTime<Utc>);

impl HttpResourceLastModifiedAt {
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    pub fn from_http_date_str(value: &str) -> Option<Self> {
        let system_time = parse_http_date(value).ok()?;
        Some(Self::new(DateTime::<Utc>::from(system_time)))
    }

    pub fn to_http_date_string(&self) -> Option<String> {
        let seconds = self.0.timestamp();
        let seconds: u64 = seconds.try_into().ok()?;
        let system_time = SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(seconds))?;
        Some(fmt_http_date(system_time))
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
