use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    AccountBalanceError, AccountCloseRejectionReason, AccountDepositRejectionReason,
    AccountDescriptionChangeRejectionReason, AccountFreezeRejectionReason,
    AccountFundsReserveRejectionReason, AccountId, AccountNameChangeRejectionReason,
    AccountOwnershipTransferRejectionReason, AccountReservedFundsCommitRejectionReason,
    AccountReservedFundsReleaseRejectionReason, AccountStateError, AccountThawRejectionReason,
    AccountWithdrawRejectionReason,
};

/// Describes why an `Account` aggregate operation failed.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error(transparent)]
    State(#[from] AccountStateError),

    #[error("account is already opened")]
    AlreadyOpened,

    #[error(transparent)]
    AccountBalance(#[from] AccountBalanceError),

    #[error("account deposit rejected: {0:?}")]
    DepositRejected(AccountDepositRejectionReason),

    #[error("account withdrawal rejected: {0:?}")]
    WithdrawRejected(AccountWithdrawRejectionReason),

    #[error("account funds reservation rejected: {0:?}")]
    FundsReserveRejected(AccountFundsReserveRejectionReason),

    #[error("account reserved funds release rejected: {0:?}")]
    ReservedFundsReleaseRejected(AccountReservedFundsReleaseRejectionReason),

    #[error("account reserved funds commit rejected: {0:?}")]
    ReservedFundsCommitRejected(AccountReservedFundsCommitRejectionReason),

    #[error("account ownership transfer rejected: {0:?}")]
    OwnershipTransferRejected(AccountOwnershipTransferRejectionReason),

    #[error("account name change rejected: {0:?}")]
    NameChangeRejected(AccountNameChangeRejectionReason),

    #[error("account description change rejected: {0:?}")]
    DescriptionChangeRejected(AccountDescriptionChangeRejectionReason),

    #[error("account freeze rejected: {0:?}")]
    FreezeRejected(AccountFreezeRejectionReason),

    #[error("account thaw rejected: {0:?}")]
    ThawRejected(AccountThawRejectionReason),

    #[error("account close rejected: {0:?}")]
    CloseRejected(AccountCloseRejectionReason),
}
