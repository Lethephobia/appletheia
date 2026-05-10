use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::AccountId;

use super::{PublicAccountListItemCurrency, PublicAccountListItemOwner};

/// Read model for one public account list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListItem {
    pub account_id: AccountId,
    pub owner: PublicAccountListItemOwner,
    pub currency: PublicAccountListItemCurrency,
    pub created_at: EventOccurredAt,
}
