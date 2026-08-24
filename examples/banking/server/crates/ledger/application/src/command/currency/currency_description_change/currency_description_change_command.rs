use appletheia::command;
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId};
use serde::{Deserialize, Serialize};

#[command(name = "currency_description_change")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyDescriptionChangeCommand {
    pub currency_id: CurrencyId,
    pub description: Option<CurrencyDescription>,
}
