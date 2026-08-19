use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;

/// Cursor for public account list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct PublicAccountListCursor {
    pub created_at: EventOccurredAt,
    pub account_id: AccountId,
}
