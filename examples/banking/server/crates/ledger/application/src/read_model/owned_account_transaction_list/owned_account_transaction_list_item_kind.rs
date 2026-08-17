use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use banking_ledger_domain::transfer::TransferId;

use super::OwnedAccountTransactionListItemCounterpartyAccountPart;

/// Kind of transaction displayed in the owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OwnedAccountTransactionListItemKind {
    Deposit,
    Withdrawal,
    Transfer {
        transfer_id: TransferId,
        counterparty_account: Box<OwnedAccountTransactionListItemCounterpartyAccountPart>,
    },
    CurrencyIssuance,
}

impl ReadModelObservationSource for OwnedAccountTransactionListItemKind {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::Transfer {
                counterparty_account,
                ..
            } => counterparty_account.observations(),
            Self::Deposit | Self::Withdrawal | Self::CurrencyIssuance => Vec::new(),
        }
    }
}
