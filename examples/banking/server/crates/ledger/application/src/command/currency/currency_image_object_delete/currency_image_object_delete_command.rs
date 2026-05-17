use appletheia::command;
use banking_ledger_domain::currency::CurrencyImageObjectName;
use serde::{Deserialize, Serialize};

/// Deletes a currency image object.
#[command(name = "currency_image_object_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyImageObjectDeleteCommand {
    pub object_name: CurrencyImageObjectName,
}
