use super::CurrencyLifecycleRejectionReason;

/// Describes the outcome of a Currency lifecycle command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrencyLifecycleResult {
    Changed,
    Rejected {
        reason: CurrencyLifecycleRejectionReason,
    },
}
