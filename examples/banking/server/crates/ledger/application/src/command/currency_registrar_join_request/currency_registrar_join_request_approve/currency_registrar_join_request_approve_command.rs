use appletheia::command;
use banking_ledger_domain::CurrencyRegistrarJoinRequestId;
use serde::{Deserialize, Serialize};

/// Approves an currency registrar join request.
#[command(name = "currency_registrar_join_request_approve")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestApproveCommand {
    pub currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
}
