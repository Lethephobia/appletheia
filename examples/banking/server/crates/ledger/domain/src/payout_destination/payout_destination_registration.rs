use super::{PayoutDestinationOwner, TokenAccountOwnerAddress};

/// Describes a payout destination registration request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayoutDestinationRegistration {
    pub owner: PayoutDestinationOwner,
    pub token_account_owner_address: TokenAccountOwnerAddress,
}

impl PayoutDestinationRegistration {
    pub(super) fn into_parts(self) -> (PayoutDestinationOwner, TokenAccountOwnerAddress) {
        (self.owner, self.token_account_owner_address)
    }
}
