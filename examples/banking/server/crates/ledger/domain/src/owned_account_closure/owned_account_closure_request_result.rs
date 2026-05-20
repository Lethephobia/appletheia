use super::OwnedAccountClosureId;

/// Describes the domain outcome of an owned account closure request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnedAccountClosureRequestResult {
    Requested {
        owned_account_closure_id: OwnedAccountClosureId,
    },
}
