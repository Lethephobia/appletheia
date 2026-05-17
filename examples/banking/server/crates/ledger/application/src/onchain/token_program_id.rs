use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::{
    CurrencyMintTokenProgramId, CurrencyMintTokenProgramIdError,
};
use serde::{Deserialize, Serialize};

use super::TokenProgramIdError;

/// Identifies an on-chain token program used by an on-chain gateway.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TokenProgramId(String);

impl TokenProgramId {
    /// Creates a token program ID.
    pub fn new(value: String) -> Result<Self, TokenProgramIdError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(TokenProgramIdError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(TokenProgramIdError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the token program ID value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TokenProgramId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for TokenProgramId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for TokenProgramId {
    type Err = TokenProgramIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for TokenProgramId {
    type Error = TokenProgramIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for TokenProgramId {
    type Error = TokenProgramIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<TokenProgramId> for CurrencyMintTokenProgramId {
    type Error = CurrencyMintTokenProgramIdError;

    fn try_from(value: TokenProgramId) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

impl From<TokenProgramId> for String {
    fn from(value: TokenProgramId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyMintTokenProgramId;

    use super::{TokenProgramId, TokenProgramIdError};

    #[test]
    fn accepts_valid_token_program_id() {
        let token_program_id =
            TokenProgramId::try_from("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
                .expect("token program ID should be valid");

        assert_eq!(
            token_program_id.value(),
            "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        );
    }

    #[test]
    fn converts_to_domain_currency_mint_token_program_id() {
        let token_program_id =
            TokenProgramId::try_from("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
                .expect("token program ID should be valid");

        let domain_token_program_id = CurrencyMintTokenProgramId::try_from(token_program_id)
            .expect("domain token program ID should be valid");

        assert_eq!(
            domain_token_program_id.value(),
            "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        );
    }

    #[test]
    fn rejects_empty_token_program_id() {
        let error = TokenProgramId::try_from(" ").expect_err("empty ID should fail");

        assert!(matches!(error, TokenProgramIdError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_token_program_id() {
        let error = TokenProgramId::try_from("Token 2022").expect_err("invalid ID should fail");

        assert!(matches!(error, TokenProgramIdError::InvalidFormat));
    }
}
