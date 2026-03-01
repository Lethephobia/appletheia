use chrono::Duration;

use super::OidcMaxAgeError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OidcMaxAge(Duration);

impl OidcMaxAge {
    pub fn new(value: Duration) -> Result<Self, OidcMaxAgeError> {
        if value < Duration::zero() {
            return Err(OidcMaxAgeError::Negative);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}
