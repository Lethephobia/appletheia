use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::onchain::{ChainNetwork, TokenAddress};
use serde::{Deserialize, Serialize};

#[command(name = "token_binding_bind")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenBindingBindCommand {
    pub currency_id: CurrencyId,
    pub chain_network: ChainNetwork,
    pub token_address: TokenAddress,
    pub deposit_enabled: bool,
    pub withdrawal_enabled: bool,
}
