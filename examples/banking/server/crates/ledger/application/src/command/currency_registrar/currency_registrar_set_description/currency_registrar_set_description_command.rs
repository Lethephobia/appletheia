use appletheia::command;
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrarDescription, CurrencyRegistrarId,
};
use serde::{Deserialize, Serialize};

#[command(name = "currency_registrar_set_description")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarSetDescriptionCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub description: Option<CurrencyRegistrarDescription>,
}
