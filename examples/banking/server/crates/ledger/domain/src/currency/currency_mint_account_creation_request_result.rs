use super::CurrencyMintAccountCreationRequestRejectionReason;

/// Represents the outcome of requesting a currency mint account creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyMintAccountCreationRequestResult {
    Requested,
    Rejected {
        reason: CurrencyMintAccountCreationRequestRejectionReason,
    },
}
