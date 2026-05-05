use banking_ledger_domain::account::AccountId;

use super::TransferRecipientListItemCurrency;

/// Account part of a transfer recipient list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecipientListItemAccount {
    pub account_id: AccountId,
    pub currency: TransferRecipientListItemCurrency,
}
