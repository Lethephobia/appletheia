use appletheia::command;
use banking_ledger_domain::{CurrencyRegistrarId, UserId};
use serde::{Deserialize, Serialize};

/// Submits an currency registrar join request.
#[command(name = "currency_registrar_join_request_submit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarJoinRequestSubmitCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub requester_id: UserId,
}
