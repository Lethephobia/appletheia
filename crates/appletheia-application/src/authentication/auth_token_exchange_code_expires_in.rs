use chrono::Duration;

use super::AuthTokenExchangeCodeExpiresInError;

/// Defines how long an auth token exchange code remains valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeCodeExpiresIn(Duration);

impl AuthTokenExchangeCodeExpiresIn {
    /// Creates a positive exchange code lifetime.
    pub fn new(value: Duration) -> Result<Self, AuthTokenExchangeCodeExpiresInError> {
        if value <= Duration::zero() {
            return Err(AuthTokenExchangeCodeExpiresInError::NonPositive);
        }
        Ok(Self(value))
    }

    /// Returns the wrapped duration.
    pub fn value(&self) -> Duration {
        self.0
    }
}
