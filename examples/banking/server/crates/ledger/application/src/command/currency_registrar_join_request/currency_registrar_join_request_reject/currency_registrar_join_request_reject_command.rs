use appletheia::command;
use banking_ledger_domain::CurrencyRegistrarJoinRequestId;
use serde::{Deserialize, Serialize};

/// Rejects an currency registrar join request.
#[command(name = "currency_registrar_join_request_reject")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestRejectCommand {
    pub currency_registrar_join_request_id: CurrencyRegistrarJoinRequestId,
}
