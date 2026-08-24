use appletheia::command;
use banking_ledger_domain::token_binding::TokenBindingId;
use serde::{Deserialize, Serialize};

#[command(name = "token_binding_deposit_enabled_change")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenBindingDepositEnabledChangeCommand {
    pub token_binding_id: TokenBindingId,
    pub enabled: bool,
}
