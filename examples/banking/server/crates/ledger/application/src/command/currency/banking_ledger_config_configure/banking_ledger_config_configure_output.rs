use serde::{Deserialize, Serialize};

/// Returned after the on-chain banking ledger config configuration request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BankingLedgerConfigConfigureOutput;
