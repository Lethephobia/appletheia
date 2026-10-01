use serde::Serialize;

use banking_ledger_domain::account::AccountId;

use super::OwnedAccountTransactionListItemCounterpartyAccountOwner;

/// Counterparty account shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountTransactionListItemCounterpartyAccount {
    pub id: AccountId,
    pub owner: OwnedAccountTransactionListItemCounterpartyAccountOwner,
}
