use appletheia::command;
use banking_iam_domain::UserId;
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;
use serde::{Deserialize, Serialize};

#[command(name = "currency_registrar_membership_create")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRegistrarMembershipCreateCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub user_id: UserId,
}
