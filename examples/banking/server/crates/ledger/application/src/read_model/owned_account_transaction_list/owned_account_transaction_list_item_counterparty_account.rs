use banking_ledger_domain::account::AccountId;

use super::OwnedAccountTransactionListItemCounterpartyAccountOwner;

/// Counterparty account shown in a transfer transaction list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListItemCounterpartyAccount {
    pub id: AccountId,
    pub owner: OwnedAccountTransactionListItemCounterpartyAccountOwner,
}
