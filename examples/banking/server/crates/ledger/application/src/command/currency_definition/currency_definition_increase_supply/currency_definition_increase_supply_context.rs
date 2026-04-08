use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Carries the workflow context for increasing currency supply.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyDefinitionIncreaseSupplyContext {
    #[default]
    Direct,
    Issuance {
        currency_issuance_id: CurrencyIssuanceId,
    },
}
