use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::mint::MintAccountSeed;

use super::MintMetadataObjectNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintMetadataObjectName(String);

impl MintMetadataObjectName {
    pub fn new(seed: &MintAccountSeed) -> Self {
        Self(format!("currencies/{}/mint/metadata.json", seed.value()))
    }

    pub fn parse(value: String) -> Result<Self, MintMetadataObjectNameError> {
        if value.is_empty() {
            return Err(MintMetadataObjectNameError::Empty);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintMetadataObjectName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintMetadataObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for MintMetadataObjectName {
    type Err = MintMetadataObjectNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value.to_owned())
    }
}

impl TryFrom<&str> for MintMetadataObjectName {
    type Error = MintMetadataObjectNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataObjectName {
    type Error = MintMetadataObjectNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<MintMetadataObjectName> for String {
    fn from(value: MintMetadataObjectName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::mint::MintAccountSeed;

    use super::{MintMetadataObjectName, MintMetadataObjectNameError};

    #[test]
    fn new_builds_deterministic_metadata_object_name() {
        let seed = MintAccountSeed::try_from("00000000000000000000000000000000")
            .expect("seed should be valid");

        let object_name = MintMetadataObjectName::new(&seed);

        assert_eq!(
            object_name.value(),
            "currencies/00000000000000000000000000000000/mint/metadata.json"
        );
    }

    #[test]
    fn rejects_empty_metadata_object_name() {
        let error = MintMetadataObjectName::try_from("")
            .expect_err("empty metadata object name should fail");

        assert!(matches!(error, MintMetadataObjectNameError::Empty));
    }
}
