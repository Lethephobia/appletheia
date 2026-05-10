use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::currency::CurrencyId;

/// Cursor for currency list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CurrencyListCursor {
    pub created_at: EventOccurredAt,
    pub currency_id: CurrencyId,
}
