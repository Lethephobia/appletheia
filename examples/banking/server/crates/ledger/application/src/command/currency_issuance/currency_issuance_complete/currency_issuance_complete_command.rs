use appletheia::command;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Completes the specified currency issuance.
#[command(name = "currency_issuance_complete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssuanceCompleteCommand {
    pub currency_issuance_id: CurrencyIssuanceId,
}
