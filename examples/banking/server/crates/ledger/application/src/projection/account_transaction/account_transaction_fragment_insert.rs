use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;

use super::{
    AccountTransactionDirection, AccountTransactionFragmentKind, AccountTransactionId,
    AccountTransactionStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountTransactionFragmentInsert {
    pub transaction_id: AccountTransactionId,
    pub account_id: AccountId,
    pub counterparty_account_id: Option<AccountId>,
    pub amount: CurrencyAmount,
    pub direction: AccountTransactionDirection,
    pub kind: AccountTransactionFragmentKind,
    pub status: AccountTransactionStatus,
}
