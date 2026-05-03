use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;

/// Cursor for account list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountListCursor {
    pub created_at: EventOccurredAt,
    pub id: AccountId,
}
