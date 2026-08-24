use super::CurrencyRegistrarMembershipRemoveRejectionReason;

/// Describes the outcome of removing a CurrencyRegistrarMembership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrencyRegistrarMembershipRemoveResult {
    Removed,
    Rejected {
        reason: CurrencyRegistrarMembershipRemoveRejectionReason,
    },
}
