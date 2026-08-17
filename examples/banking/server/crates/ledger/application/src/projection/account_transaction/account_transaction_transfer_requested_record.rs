use appletheia::application::request_context::CorrelationId;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountTransactionTransferRequestedRecord {
    pub id: TransferId,
    pub correlation_id: CorrelationId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
}
