use chrono::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OidcLoginAttemptExpiresIn(Duration);

impl OidcLoginAttemptExpiresIn {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}
