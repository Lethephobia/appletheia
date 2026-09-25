use super::CurrencyRegistrarMembershipCreateRejectionReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrencyRegistrarMembershipCreateResult {
    Created,
    Rejected {
        reason: CurrencyRegistrarMembershipCreateRejectionReason,
    },
}
