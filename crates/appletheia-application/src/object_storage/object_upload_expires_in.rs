use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::ObjectUploadExpiresInError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectUploadExpiresIn(Duration);

impl ObjectUploadExpiresIn {
    pub fn new(value: Duration) -> Result<Self, ObjectUploadExpiresInError> {
        if value <= Duration::zero() {
            return Err(ObjectUploadExpiresInError::NonPositive);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn new_rejects_non_positive_duration() {
        let error = ObjectUploadExpiresIn::new(Duration::zero())
            .expect_err("zero duration should be rejected");

        assert!(matches!(error, ObjectUploadExpiresInError::NonPositive));
    }
}
