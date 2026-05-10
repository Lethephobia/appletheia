use banking_ledger_domain::transfer::TransferId;

use super::OwnedAccountTransactionListItemCounterpartyAccount;

/// Kind of transaction displayed in the owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedAccountTransactionListItemKind {
    Deposit,
    Withdrawal,
    Transfer {
        transfer_id: TransferId,
        counterparty_account: OwnedAccountTransactionListItemCounterpartyAccount,
    },
    CurrencyIssuance,
}
