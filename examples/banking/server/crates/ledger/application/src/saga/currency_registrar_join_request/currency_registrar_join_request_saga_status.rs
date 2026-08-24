use serde::{Deserialize, Serialize};

/// Describes progress for the currency registrar join request saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyRegistrarJoinRequestSagaStatus {
    MembershipCreateRequested,
    MembershipCreated,
    AlreadyMember,
    Failed,
}
