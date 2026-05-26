use super::PayoutDestinationId;

/// Describes the domain outcome of a payout destination registration.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PayoutDestinationRegisterResult {
    Registered {
        payout_destination_id: PayoutDestinationId,
    },
}
