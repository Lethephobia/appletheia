use appletheia::command;
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrarDescription, CurrencyRegistrarDisplayName, CurrencyRegistrarHandle,
};
use serde::{Deserialize, Serialize};

/// Creates an authorization boundary for registering currencies.
#[command(name = "currency_registrar_create")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRegistrarCreateCommand {
    pub handle: CurrencyRegistrarHandle,
    pub display_name: CurrencyRegistrarDisplayName,
    pub description: Option<CurrencyRegistrarDescription>,
}
