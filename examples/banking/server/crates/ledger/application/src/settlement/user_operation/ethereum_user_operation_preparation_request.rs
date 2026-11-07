use super::EvmTransactionRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumUserOperationPreparationRequest {
    transaction_request: EvmTransactionRequest,
}

impl EthereumUserOperationPreparationRequest {
    pub const fn new(transaction_request: EvmTransactionRequest) -> Self {
        Self {
            transaction_request,
        }
    }

    pub const fn transaction_request(&self) -> &EvmTransactionRequest {
        &self.transaction_request
    }
}
