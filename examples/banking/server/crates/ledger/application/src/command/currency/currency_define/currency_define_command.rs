use appletheia::command;
use banking_ledger_domain::core::{CurrencyCode, CurrencyDecimals};
use banking_ledger_domain::currency::CurrencyDescription;
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;
use serde::{Deserialize, Serialize};

#[command(name = "currency_define")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyDefineCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub code: CurrencyCode,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
}
