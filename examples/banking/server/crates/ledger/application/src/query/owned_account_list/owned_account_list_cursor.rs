use banking_ledger_domain::account::AccountId;

/// Cursor for account list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountListCursor {
    pub id: AccountId,
}
