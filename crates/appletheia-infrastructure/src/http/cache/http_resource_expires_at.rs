use chrono::{DateTime, Utc};
use httpdate::parse_http_date;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpResourceExpiresAt(DateTime<Utc>);

impl HttpResourceExpiresAt {
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    pub fn from_cache_headers(
        now: DateTime<Utc>,
        cache_control: Option<&str>,
        expires: Option<&str>,
    ) -> Option<Self> {
        if let Some(cache_control) = cache_control
            && let Some(max_age_seconds) = Self::parse_cache_control_max_age_seconds(cache_control)
        {
            return Some(Self::new(now + chrono::Duration::seconds(max_age_seconds)));
        }

        let expires = expires?;
        let system_time = parse_http_date(expires).ok()?;
        Some(Self::new(DateTime::<Utc>::from(system_time)))
    }

    fn parse_cache_control_max_age_seconds(value: &str) -> Option<i64> {
        for directive in value.split(',') {
            let directive = directive.trim();
            let Some(rest) = directive.strip_prefix("max-age=") else {
                continue;
            };
            if let Ok(parsed) = rest.parse::<i64>()
                && parsed >= 0
            {
                return Some(parsed);
            }
        }
        None
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
