use super::TokenBindingEnablementChangeRejectionReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenBindingEnablementChangeResult {
    Changed,
    Rejected {
        reason: TokenBindingEnablementChangeRejectionReason,
    },
}
