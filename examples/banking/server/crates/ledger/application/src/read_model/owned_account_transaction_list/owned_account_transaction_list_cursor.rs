use appletheia::domain::EventOccurredAt;

use crate::projection::AccountTransactionId;

/// Cursor for owned account transaction list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountTransactionListCursor {
    pub occurred_at: EventOccurredAt,
    pub transaction_id: AccountTransactionId,
}
