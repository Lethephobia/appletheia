use appletheia::domain::EventId;
use banking_ledger_domain::transfer::TransferId;

use super::OwnedAccountTransactionListItemCounterpartyAccount;

/// Kind of transaction displayed in the owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedAccountTransactionListItemKind {
    Deposit,
    Withdrawal,
    Transfer {
        transfer_id: TransferId,
        counterparty_account: Box<OwnedAccountTransactionListItemCounterpartyAccount>,
    },
    CurrencyIssuance,
}

impl OwnedAccountTransactionListItemKind {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        match self {
            Self::Transfer {
                counterparty_account,
                ..
            } => counterparty_account.observed_event_ids(),
            Self::Deposit | Self::Withdrawal | Self::CurrencyIssuance => Vec::new(),
        }
    }
}
