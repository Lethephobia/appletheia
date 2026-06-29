use appletheia::command;
use serde::{Deserialize, Serialize};

/// Configures the on-chain banking ledger config.
#[command(name = "banking_ledger_config_configure")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BankingLedgerConfigConfigureCommand;
