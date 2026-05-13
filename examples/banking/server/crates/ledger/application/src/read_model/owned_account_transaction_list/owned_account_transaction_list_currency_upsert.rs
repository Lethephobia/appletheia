use appletheia::application::event::EventSequence;
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListCurrencyUpsert {
    pub id: CurrencyId,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}
