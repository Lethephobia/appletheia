use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Provisions a currency by creating its on-chain mint account.
#[command(name = "currency_provision")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyProvisionCommand {
    pub currency_id: CurrencyId,
}
