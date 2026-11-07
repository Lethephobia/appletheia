use appletheia::command;
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrarDisplayName, CurrencyRegistrarId,
};
use serde::{Deserialize, Serialize};

#[command(name = "currency_registrar_change_display_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarChangeDisplayNameCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub display_name: CurrencyRegistrarDisplayName,
}
