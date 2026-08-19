use serde::Serialize;

use appletheia::domain::EventOccurredAt;

use super::OwnedAccountTransactionId;

/// Cursor for owned account transaction list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct OwnedAccountTransactionListCursor {
    pub occurred_at: EventOccurredAt,
    pub transaction_id: OwnedAccountTransactionId,
}
