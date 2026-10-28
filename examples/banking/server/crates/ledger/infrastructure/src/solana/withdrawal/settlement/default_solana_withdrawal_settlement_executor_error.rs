use thiserror::Error;

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum DefaultSolanaWithdrawalSettlementExecutorError {
    #[error("withdrawal settlement receipt has an unexpected owner")]
    UnexpectedReceiptOwner,

    #[error("withdrawal settlement receipt does not match the request")]
    ReceiptMismatch,

    #[error("successful withdrawal transaction could not be recovered")]
    SuccessfulTransactionNotFound,
}
