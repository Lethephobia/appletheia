use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::AuthTokenExpiresInError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenExpiresIn(Duration);

impl AuthTokenExpiresIn {
    pub fn new(value: Duration) -> Result<Self, AuthTokenExpiresInError> {
        if value <= Duration::zero() {
            return Err(AuthTokenExpiresInError::NonPositive);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}
