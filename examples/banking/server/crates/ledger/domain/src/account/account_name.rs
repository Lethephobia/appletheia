use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::AccountNameError;

/// Represents a validated account name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountName(String);

impl AccountName {
    /// Creates an account name from user input.
    pub fn new(value: String) -> Result<Self, AccountNameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(AccountNameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(AccountNameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AccountName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for AccountName {
    type Err = AccountNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for AccountName {
    type Error = AccountNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for AccountName {
    type Error = AccountNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AccountName> for String {
    fn from(value: AccountName) -> Self {
        value.0
    }
}
