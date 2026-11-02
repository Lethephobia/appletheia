use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

#[command(name = "currency_deactivate")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyDeactivateCommand {
    pub currency_id: CurrencyId,
}
