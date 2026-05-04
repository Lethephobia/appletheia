use appletheia::domain::{EventId, EventOccurredAt};

/// Cursor for owned account transaction list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountTransactionListItemCursor {
    pub occurred_at: EventOccurredAt,
    pub id: EventId,
}
