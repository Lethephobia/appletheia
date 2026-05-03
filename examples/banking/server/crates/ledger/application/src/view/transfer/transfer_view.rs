use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::{TransferId, TransferStatus};

/// Represents a normalized transfer view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferView {
    pub id: TransferId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub status: TransferStatus,
}
