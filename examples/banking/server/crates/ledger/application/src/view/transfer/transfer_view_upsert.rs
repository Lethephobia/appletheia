use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::{TransferId, TransferStatus};

/// Attributes required to upsert a normalized transfer view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferViewUpsert {
    pub id: TransferId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub status: TransferStatus,
}
