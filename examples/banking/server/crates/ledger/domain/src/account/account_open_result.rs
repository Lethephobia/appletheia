use super::AccountId;

/// Describes the domain outcome of an account open request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountOpenResult {
    Opened { account_id: AccountId },
}
