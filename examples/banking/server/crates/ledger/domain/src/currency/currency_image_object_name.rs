use std::fmt::{self, Display};
use std::str::FromStr;

use appletheia::domain::AggregateId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CurrencyId, CurrencyImageObjectNameError};

/// Represents a currency image object name in object storage.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyImageObjectName(String);

impl CurrencyImageObjectName {
    /// Creates a new image object name for the given currency.
    pub fn new(currency_id: CurrencyId) -> Self {
        Self(format!(
            "currencies/{}/images/{}",
            currency_id.value(),
            Uuid::now_v7()
        ))
    }

    /// Returns the image object name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyImageObjectName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyImageObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencyImageObjectName {
    type Err = CurrencyImageObjectNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(CurrencyImageObjectNameError::Empty);
        }

        let segments = value.split('/').collect::<Vec<_>>();
        if segments.len() != 4 || segments[0] != "currencies" || segments[2] != "images" {
            return Err(CurrencyImageObjectNameError::InvalidFormat);
        }

        CurrencyId::try_from_uuid(
            Uuid::parse_str(segments[1])
                .map_err(|_| CurrencyImageObjectNameError::InvalidFormat)?,
        )
        .map_err(|_| CurrencyImageObjectNameError::InvalidFormat)?;
        Uuid::parse_str(segments[3]).map_err(|_| CurrencyImageObjectNameError::InvalidFormat)?;

        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<&str> for CurrencyImageObjectName {
    type Error = CurrencyImageObjectNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyImageObjectName {
    type Error = CurrencyImageObjectNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<CurrencyImageObjectName> for String {
    fn from(value: CurrencyImageObjectName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::{CurrencyId, CurrencyImageObjectName, CurrencyImageObjectNameError};

    #[test]
    fn new_generates_image_object_name_for_currency() {
        let currency_id = CurrencyId::try_from_uuid(Uuid::nil()).expect("currency ID is valid");
        let object_name = CurrencyImageObjectName::new(currency_id);
        let segments = object_name.value().split('/').collect::<Vec<_>>();

        assert_eq!(segments.len(), 4);
        assert_eq!(segments[0], "currencies");
        assert_eq!(segments[1], "00000000-0000-0000-0000-000000000000");
        assert_eq!(segments[2], "images");
        Uuid::parse_str(segments[3]).expect("image ID should be a UUID");
    }

    #[test]
    fn try_from_accepts_valid_image_object_name() {
        let object_name = CurrencyImageObjectName::try_from(
            "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
        )
        .expect("name should be valid");

        assert_eq!(
            object_name.value(),
            "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
        );
    }

    #[test]
    fn try_from_rejects_invalid_image_object_name() {
        let error = CurrencyImageObjectName::try_from("currencies/not-a-uuid/images/not-a-uuid")
            .expect_err("name should be invalid");

        assert!(matches!(error, CurrencyImageObjectNameError::InvalidFormat));
    }
}
