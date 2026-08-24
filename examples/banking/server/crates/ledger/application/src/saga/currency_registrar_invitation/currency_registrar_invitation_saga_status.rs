use serde::{Deserialize, Serialize};

/// Describes progress for the currency registrar invitation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyRegistrarInvitationSagaStatus {
    MembershipCreateRequested,
    MembershipCreated,
    AlreadyMember,
    Failed,
}
