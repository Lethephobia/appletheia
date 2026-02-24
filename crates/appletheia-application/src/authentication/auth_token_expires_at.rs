use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::AuthTokenExpiresIn;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AuthTokenExpiresAt(DateTime<Utc>);

impl AuthTokenExpiresAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_now(expires_in: AuthTokenExpiresIn) -> Self {
        Self(Utc::now() + expires_in.value())
    }
}

impl Default for AuthTokenExpiresAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for AuthTokenExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenExpiresAt> for DateTime<Utc> {
    fn from(value: AuthTokenExpiresAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
