use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use super::SignedObjectUploadUrlError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignedObjectUploadUrl(Url);

impl SignedObjectUploadUrl {
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for SignedObjectUploadUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for SignedObjectUploadUrl {
    type Err = SignedObjectUploadUrlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value).map_err(SignedObjectUploadUrlError::Parse)?;
        Ok(Self(url))
    }
}

impl TryFrom<&str> for SignedObjectUploadUrl {
    type Error = SignedObjectUploadUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SignedObjectUploadUrl {
    type Error = SignedObjectUploadUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_rejects_invalid_url() {
        assert!(matches!(
            "not a url".parse::<SignedObjectUploadUrl>(),
            Err(SignedObjectUploadUrlError::Parse(_))
        ));
    }
}
