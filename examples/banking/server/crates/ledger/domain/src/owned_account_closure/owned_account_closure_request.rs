use crate::account::AccountOwner;

/// Describes an owned account closure request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OwnedAccountClosureRequest {
    pub owner: AccountOwner,
}

impl OwnedAccountClosureRequest {
    pub(super) fn into_owner(self) -> AccountOwner {
        self.owner
    }
}
