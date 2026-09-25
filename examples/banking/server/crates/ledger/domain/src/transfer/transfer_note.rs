use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::TransferNoteError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TransferNote(String);

impl TransferNote {
    pub fn new(value: String) -> Result<Self, TransferNoteError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(TransferNoteError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(TransferNoteError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TransferNote {
    fn as_ref(&self) -> &str {
        self.value()
    }
}
impl Display for TransferNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}
impl FromStr for TransferNote {
    type Err = TransferNoteError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}
impl TryFrom<&str> for TransferNote {
    type Error = TransferNoteError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
impl TryFrom<String> for TransferNote {
    type Error = TransferNoteError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<TransferNote> for String {
    fn from(value: TransferNote) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{TransferNote, TransferNoteError};

    #[test]
    fn validates_transfer_notes_independently() {
        assert_eq!(
            TransferNote::try_from(" ").expect_err("empty note should fail"),
            TransferNoteError::Empty
        );
    }
}
