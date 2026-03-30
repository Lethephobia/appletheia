use appletheia::command;
use banking_iam_domain::UserId;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use serde::{Deserialize, Serialize};

/// Opens a new account for the specified user.
#[command(name = "account_open")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountOpenCommand {
    pub user_id: UserId,
    pub currency_definition_id: CurrencyDefinitionId,
}
