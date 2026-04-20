use appletheia::command;
use banking_ledger_domain::currency::{CurrencyId, CurrencyOwner};
use serde::{Deserialize, Serialize};

/// Transfers ownership of a currency.
#[command(name = "currency_ownership_transfer")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyOwnershipTransferCommand {
    pub currency_id: CurrencyId,
    pub owner: CurrencyOwner,
}
