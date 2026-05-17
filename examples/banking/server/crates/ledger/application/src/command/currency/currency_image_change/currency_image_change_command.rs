use appletheia::command;
use banking_ledger_domain::currency::{CurrencyId, CurrencyImageRef};
use serde::{Deserialize, Serialize};

/// Changes a currency image reference.
#[command(name = "currency_image_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyImageChangeCommand {
    pub currency_id: CurrencyId,
    pub image: Option<CurrencyImageRef>,
}
