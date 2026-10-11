use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationLedgerRole {
    FinanceManager,
    Treasurer,
}

impl OrganizationLedgerRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinanceManager => "finance_manager",
            Self::Treasurer => "treasurer",
        }
    }
}
