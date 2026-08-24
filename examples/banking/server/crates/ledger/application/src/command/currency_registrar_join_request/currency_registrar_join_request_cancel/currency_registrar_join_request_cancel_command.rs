use appletheia::command;
use banking_ledger_domain::CurrencyRegistrarJoinRequestId;
use serde::{Deserialize, Serialize};

/// Cancels an currency registrar join request.
#[command(name = "currency_registrar_join_request_cancel")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestCancelCommand {
    pub currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
}
