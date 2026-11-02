use thiserror::Error;

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum DefaultSolanaDepositSettlementVerifierError {
    #[error("deposit transaction did not create the expected settlement receipt")]
    ExpectedReceiptNotCreated,

    #[error("deposit settlement receipt has an unexpected owner")]
    UnexpectedReceiptOwner,

    #[error("deposit settlement receipt does not match the request")]
    ReceiptMismatch,
}
