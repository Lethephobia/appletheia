use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Synchronizes on-chain mint supply for a currency into the internal pool account.
#[command(name = "mint_supply_sync")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintSupplySyncCommand {
    pub currency_id: CurrencyId,
}
