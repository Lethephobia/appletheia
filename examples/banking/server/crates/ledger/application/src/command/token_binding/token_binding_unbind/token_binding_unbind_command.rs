use appletheia::command;
use banking_ledger_domain::token_binding::TokenBindingId;
use serde::{Deserialize, Serialize};

#[command(name = "token_binding_unbind")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenBindingUnbindCommand {
    pub token_binding_id: TokenBindingId,
}
