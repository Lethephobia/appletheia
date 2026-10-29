use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenBindingStatus {
    Active,
    Unbound,
}

impl TokenBindingStatus {
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    pub const fn is_unbound(self) -> bool {
        matches!(self, Self::Unbound)
    }
}
