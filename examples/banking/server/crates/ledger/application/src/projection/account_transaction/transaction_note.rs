use std::fmt::{self, Display};

use banking_ledger_domain::deposit::DepositNote;
use banking_ledger_domain::transfer::TransferNote;
use banking_ledger_domain::withdrawal::WithdrawalNote;
use serde::{Deserialize, Serialize};

use super::TransactionNoteError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TransactionNote(String);

impl TransactionNote {
    pub fn new(value: String) -> Result<Self, TransactionNoteError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(TransactionNoteError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(TransactionNoteError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TransactionNote {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for TransactionNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl TryFrom<String> for TransactionNote {
    type Error = TransactionNoteError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<TransactionNote> for String {
    fn from(value: TransactionNote) -> Self {
        value.0
    }
}

impl From<DepositNote> for TransactionNote {
    fn from(value: DepositNote) -> Self {
        Self(value.into())
    }
}

impl From<WithdrawalNote> for TransactionNote {
    fn from(value: WithdrawalNote) -> Self {
        Self(value.into())
    }
}

impl From<TransferNote> for TransactionNote {
    fn from(value: TransferNote) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::deposit::DepositNote;
    use banking_ledger_domain::transfer::TransferNote;
    use banking_ledger_domain::withdrawal::WithdrawalNote;

    use super::TransactionNote;

    #[test]
    fn converts_each_domain_note_into_the_projection_note() {
        let deposit = TransactionNote::from(
            DepositNote::try_from("deposit").expect("deposit note should be valid"),
        );
        let withdrawal = TransactionNote::from(
            WithdrawalNote::try_from("withdrawal").expect("withdrawal note should be valid"),
        );
        let transfer = TransactionNote::from(
            TransferNote::try_from("transfer").expect("transfer note should be valid"),
        );

        assert_eq!(deposit.value(), "deposit");
        assert_eq!(withdrawal.value(), "withdrawal");
        assert_eq!(transfer.value(), "transfer");
    }
}
