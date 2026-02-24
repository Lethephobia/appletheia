use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AuthTokenIssuedAt(DateTime<Utc>);

impl AuthTokenIssuedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for AuthTokenIssuedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for AuthTokenIssuedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenIssuedAt> for DateTime<Utc> {
    fn from(value: AuthTokenIssuedAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenIssuedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
