use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Carries the workflow context for `AccountDepositCommand`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountDepositContext {
    #[default]
    Direct,
    Transfer {
        transfer_id: TransferId,
        from_account_id: AccountId,
    },
    Issuance {
        currency_issuance_id: CurrencyIssuanceId,
        currency_definition_id: CurrencyDefinitionId,
    },
}
