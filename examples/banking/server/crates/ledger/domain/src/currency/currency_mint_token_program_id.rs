use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyMintTokenProgramIdError;

/// Identifies the token program used by a currency mint.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyMintTokenProgramId(String);

impl CurrencyMintTokenProgramId {
    /// Creates a currency mint token program ID.
    pub fn new(value: String) -> Result<Self, CurrencyMintTokenProgramIdError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(CurrencyMintTokenProgramIdError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(CurrencyMintTokenProgramIdError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the token program ID value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyMintTokenProgramId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyMintTokenProgramId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencyMintTokenProgramId {
    type Err = CurrencyMintTokenProgramIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyMintTokenProgramId {
    type Error = CurrencyMintTokenProgramIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyMintTokenProgramId {
    type Error = CurrencyMintTokenProgramIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyMintTokenProgramId> for String {
    fn from(value: CurrencyMintTokenProgramId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyMintTokenProgramId, CurrencyMintTokenProgramIdError};

    #[test]
    fn accepts_valid_token_program_id() {
        let token_program_id =
            CurrencyMintTokenProgramId::try_from("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
                .expect("token program ID should be valid");

        assert_eq!(
            token_program_id.value(),
            "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        );
    }

    #[test]
    fn rejects_empty_token_program_id() {
        let error = CurrencyMintTokenProgramId::try_from(" ").expect_err("empty ID should fail");

        assert!(matches!(error, CurrencyMintTokenProgramIdError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_token_program_id() {
        let error =
            CurrencyMintTokenProgramId::try_from("Token 2022").expect_err("invalid ID should fail");

        assert!(matches!(
            error,
            CurrencyMintTokenProgramIdError::InvalidFormat
        ));
    }
}
