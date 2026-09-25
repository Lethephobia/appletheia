use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::WithdrawalNoteError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WithdrawalNote(String);

impl WithdrawalNote {
    pub fn new(value: String) -> Result<Self, WithdrawalNoteError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(WithdrawalNoteError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(WithdrawalNoteError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WithdrawalNote {
    fn as_ref(&self) -> &str {
        self.value()
    }
}
impl Display for WithdrawalNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}
impl FromStr for WithdrawalNote {
    type Err = WithdrawalNoteError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}
impl TryFrom<&str> for WithdrawalNote {
    type Error = WithdrawalNoteError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
impl TryFrom<String> for WithdrawalNote {
    type Error = WithdrawalNoteError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<WithdrawalNote> for String {
    fn from(value: WithdrawalNote) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{WithdrawalNote, WithdrawalNoteError};

    #[test]
    fn validates_withdrawal_notes_independently() {
        assert_eq!(
            WithdrawalNote::try_from(" ").expect_err("empty note should fail"),
            WithdrawalNoteError::Empty
        );
    }
}
