use serde::{Deserialize, Serialize};

use crate::core::{ChainNetwork, TokenAddress};
use crate::currency::CurrencyId;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TokenBindingDefinition {
    pub currency_id: CurrencyId,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
    pub deposit_enabled: bool,
    pub withdrawal_enabled: bool,
}
