use banking_ledger_domain::core::{ChainNetwork, TokenAddress};
use banking_ledger_domain::token_binding::TokenBindingId;
use serde::{Deserialize, Serialize};

/// Materialized active TokenBinding associated with one Currency projection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyTokenBindingFragment {
    pub id: TokenBindingId,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
}
