use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{ChainNetwork, CurrencyAmount, TokenAddress};
use banking_ledger_domain::token_binding::TokenBindingId;

use super::{
    AccountTransactionDirection, AccountTransactionFragmentKind, AccountTransactionId,
    AccountTransactionStatus, TransactionNote,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountTransactionFragmentInsert {
    pub transaction_id: AccountTransactionId,
    pub account_id: AccountId,
    pub counterparty_account_id: Option<AccountId>,
    pub token_binding_id: Option<TokenBindingId>,
    pub chain_network: Option<ChainNetwork>,
    pub token_address: Option<TokenAddress>,
    pub amount: CurrencyAmount,
    pub note: Option<TransactionNote>,
    pub direction: AccountTransactionDirection,
    pub kind: AccountTransactionFragmentKind,
    pub status: AccountTransactionStatus,
}
