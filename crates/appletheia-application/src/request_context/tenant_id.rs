use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::TenantIdError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TenantId(String);

impl TenantId {
    pub const MAX_LENGTH: usize = 200;

    pub fn new(value: String) -> Result<Self, TenantIdError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), TenantIdError> {
        if value.is_empty() {
            return Err(TenantIdError::Empty);
        }
        let len = value.len();
        if len > Self::MAX_LENGTH {
            return Err(TenantIdError::TooLong {
                len,
                max: Self::MAX_LENGTH,
            });
        }
        Ok(())
    }
}

impl TryFrom<String> for TenantId {
    type Error = TenantIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TenantId {
    type Error = TenantIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        let err = TenantId::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, TenantIdError::Empty));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(TenantId::MAX_LENGTH + 1);
        let err = TenantId::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, TenantIdError::TooLong { .. }));
    }

    #[test]
    fn accepts_non_empty() {
        let id = TenantId::try_from("tenant-a").expect("non-empty");
        assert_eq!(id.value(), "tenant-a");
    }
}
