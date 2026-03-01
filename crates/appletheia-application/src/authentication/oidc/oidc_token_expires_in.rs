use chrono::Duration;

use super::OidcTokenExpiresInError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OidcTokenExpiresIn(Duration);

impl OidcTokenExpiresIn {
    pub fn new(value: Duration) -> Result<Self, OidcTokenExpiresInError> {
        if value <= Duration::zero() {
            return Err(OidcTokenExpiresInError::NonPositive);
        }
        Ok(Self(value))
    }

    pub fn from_seconds(value: u64) -> Result<Self, OidcTokenExpiresInError> {
        let Ok(seconds) = i64::try_from(value) else {
            return Err(OidcTokenExpiresInError::TooLarge);
        };
        Self::new(Duration::seconds(seconds))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}
