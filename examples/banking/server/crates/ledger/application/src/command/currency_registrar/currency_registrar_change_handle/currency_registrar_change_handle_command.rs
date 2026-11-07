use appletheia::command;
use banking_ledger_domain::currency_registrar::{CurrencyRegistrarHandle, CurrencyRegistrarId};
use serde::{Deserialize, Serialize};

#[command(name = "currency_registrar_change_handle")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarChangeHandleCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub handle: CurrencyRegistrarHandle,
}
