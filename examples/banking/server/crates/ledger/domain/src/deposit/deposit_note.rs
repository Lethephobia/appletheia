use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::DepositNoteError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct DepositNote(String);

impl DepositNote {
    pub fn new(value: String) -> Result<Self, DepositNoteError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(DepositNoteError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(DepositNoteError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DepositNote {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for DepositNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for DepositNote {
    type Err = DepositNoteError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for DepositNote {
    type Error = DepositNoteError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for DepositNote {
    type Error = DepositNoteError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<DepositNote> for String {
    fn from(value: DepositNote) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{DepositNote, DepositNoteError};

    #[test]
    fn validates_deposit_notes_independently() {
        assert_eq!(
            DepositNote::try_from(" ").expect_err("empty note should fail"),
            DepositNoteError::Empty
        );
    }
}
