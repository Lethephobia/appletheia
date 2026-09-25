use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::AccountDescriptionError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AccountDescription(String);

impl AccountDescription {
    pub fn new(value: String) -> Result<Self, AccountDescriptionError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(AccountDescriptionError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(AccountDescriptionError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AccountDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for AccountDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for AccountDescription {
    type Err = AccountDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for AccountDescription {
    type Error = AccountDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for AccountDescription {
    type Error = AccountDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AccountDescription> for String {
    fn from(value: AccountDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{AccountDescription, AccountDescriptionError};

    #[test]
    fn validates_and_normalizes_account_descriptions() {
        let description = AccountDescription::try_from("  operating account  ")
            .expect("description should be valid");
        assert_eq!(description.value(), "operating account");
        assert_eq!(
            AccountDescription::try_from(" ").expect_err("empty description should fail"),
            AccountDescriptionError::Empty
        );
    }
}
