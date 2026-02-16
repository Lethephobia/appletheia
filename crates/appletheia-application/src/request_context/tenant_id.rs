use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::TenantIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TenantId(Uuid);

impl TenantId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TenantId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for TenantId {
    type Err = TenantIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::try_parse(s)
            .map(Self)
            .map_err(|source| TenantIdError::InvalidUuid {
                value: s.to_owned(),
                source,
            })
    }
}

impl TryFrom<&str> for TenantId {
    type Error = TenantIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for TenantId {
    type Error = TenantIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::try_parse(&value)
            .map(Self)
            .map_err(|source| TenantIdError::InvalidUuid { value, source })
    }
}

impl Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_uuid() {
        let err = TenantId::try_from("not-a-uuid").expect_err("invalid should be rejected");
        assert!(matches!(err, TenantIdError::InvalidUuid { .. }));
    }

    #[test]
    fn accepts_uuid() {
        let uuid = Uuid::nil();
        let id = TenantId::from(uuid);
        assert_eq!(id.value(), uuid);
    }
}
