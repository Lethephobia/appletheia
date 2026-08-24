use appletheia::command;
use banking_ledger_domain::core::{ChainNetwork, TokenAddress};
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

#[command(name = "token_binding_define")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenBindingDefineCommand {
    pub currency_id: CurrencyId,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
}
