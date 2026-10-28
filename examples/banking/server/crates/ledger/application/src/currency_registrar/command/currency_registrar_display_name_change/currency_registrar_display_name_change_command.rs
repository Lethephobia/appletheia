use appletheia::command;
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrarDisplayName, CurrencyRegistrarId,
};
use serde::{Deserialize, Serialize};

#[command(name = "currency_registrar_display_name_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarDisplayNameChangeCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub display_name: CurrencyRegistrarDisplayName,
}
