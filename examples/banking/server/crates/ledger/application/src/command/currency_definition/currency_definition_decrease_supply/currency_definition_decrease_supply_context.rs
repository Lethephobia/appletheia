use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use serde::{Deserialize, Serialize};

/// Carries the workflow context for decreasing currency supply.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyDefinitionDecreaseSupplyContext {
    #[default]
    Direct,
    Issuance {
        currency_issuance_id: CurrencyIssuanceId,
    },
}
