use appletheia::command;
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId};
use serde::{Deserialize, Serialize};

/// Changes a currency description.
#[command(name = "currency_description_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyDescriptionChangeCommand {
    pub currency_id: CurrencyId,
    pub description: Option<CurrencyDescription>,
}
