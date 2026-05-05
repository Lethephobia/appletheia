use appletheia::query;
use banking_ledger_domain::currency::CurrencyId;

use crate::pagination::{CursorOptions, PageLimit};
use crate::read_model::{TransferRecipientListItemCursor, TransferRecipientListItemSortKey};

/// Query parameters for transfer recipient list reads.
#[query(name = "transfer_recipient_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecipientListQuery {
    pub keyword: Option<String>,
    pub currency_id: Option<CurrencyId>,
    pub cursor_options:
        Option<CursorOptions<TransferRecipientListItemSortKey, TransferRecipientListItemCursor>>,
    pub limit: PageLimit,
}
