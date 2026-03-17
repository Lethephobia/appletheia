use std::fmt::{self, Display};
use std::str::FromStr;

use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

/// Represents the OIDC `zoneinfo` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OidcZoneinfo(Tz);

impl OidcZoneinfo {
    /// Creates an OIDC zoneinfo claim value.
    pub fn new(value: Tz) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> Tz {
        self.0
    }
}

impl Display for OidcZoneinfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for OidcZoneinfo {
    type Err = chrono_tz::ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl TryFrom<String> for OidcZoneinfo {
    type Error = chrono_tz::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<OidcZoneinfo> for String {
    fn from(value: OidcZoneinfo) -> Self {
        value.to_string()
    }
}
