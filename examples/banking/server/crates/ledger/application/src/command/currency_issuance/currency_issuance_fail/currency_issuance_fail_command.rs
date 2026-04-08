use appletheia::command;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Fails the specified currency issuance.
#[command(name = "currency_issuance_fail")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceFailCommand {
    pub currency_issuance_id: CurrencyIssuanceId,
}
