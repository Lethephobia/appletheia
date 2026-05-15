use appletheia::application::event::EventSequence;
use appletheia::application::request_context::CorrelationId;
use appletheia::domain::EventId;
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListTransferRequestedRecord {
    pub id: TransferId,
    pub correlation_id: CorrelationId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub event_id: EventId,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}
