use appletheia::application::request_context::CorrelationId;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;

use super::{
    OwnedAccountTransactionId, OwnedAccountTransactionListItemDirection,
    OwnedAccountTransactionListItemKind, OwnedAccountTransactionListItemStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListItemInsert {
    pub transaction_id: OwnedAccountTransactionId,
    pub correlation_id: CorrelationId,
    pub account_id: AccountId,
    pub counterparty_account_id: Option<AccountId>,
    pub amount: CurrencyAmount,
    pub direction: OwnedAccountTransactionListItemDirection,
    pub kind: OwnedAccountTransactionListItemKind,
    pub status: OwnedAccountTransactionListItemStatus,
}
