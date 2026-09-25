use super::EvmTransactionRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumUserOperationPrepareRequest {
    transaction_request: EvmTransactionRequest,
}

impl EthereumUserOperationPrepareRequest {
    pub const fn new(transaction_request: EvmTransactionRequest) -> Self {
        Self {
            transaction_request,
        }
    }

    pub const fn transaction_request(&self) -> &EvmTransactionRequest {
        &self.transaction_request
    }
}
