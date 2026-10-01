use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, OnchainTransactionId, TokenAddress,
};

use super::{
    OwnedAccountTransactionId, OwnedAccountTransactionListItemCurrency,
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus,
};
use crate::projection::TransactionNote;

/// Read model for one owned account transaction list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountTransactionListItem {
    pub transaction_id: OwnedAccountTransactionId,
    pub account_id: AccountId,
    pub currency: OwnedAccountTransactionListItemCurrency,
    pub chain_network: Option<ChainNetwork>,
    pub token_address: Option<TokenAddress>,
    pub onchain_transaction_id: Option<OnchainTransactionId>,
    pub amount: CurrencyAmount,
    pub note: Option<TransactionNote>,
    pub direction: OwnedAccountTransactionListItemDirection,
    pub kind: OwnedAccountTransactionListItemKind,
    pub status: OwnedAccountTransactionListItemStatus,
    pub occurred_at: EventOccurredAt,
    pub created_at: EventOccurredAt,
}
