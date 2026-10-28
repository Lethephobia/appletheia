use appletheia::command;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembershipId;
use serde::{Deserialize, Serialize};

#[command(name = "currency_registrar_membership_remove")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRegistrarMembershipRemoveCommand {
    pub currency_registrar_membership_id: CurrencyRegistrarMembershipId,
}
