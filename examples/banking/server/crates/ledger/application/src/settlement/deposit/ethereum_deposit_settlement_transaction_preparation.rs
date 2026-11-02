use super::EvmTransactionRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumDepositSettlementTransactionPreparation {
    transaction_request: EvmTransactionRequest,
}

impl EthereumDepositSettlementTransactionPreparation {
    pub const fn new(transaction_request: EvmTransactionRequest) -> Self {
        Self {
            transaction_request,
        }
    }

    pub const fn transaction_request(&self) -> &EvmTransactionRequest {
        &self.transaction_request
    }

    pub fn into_transaction_request(self) -> EvmTransactionRequest {
        self.transaction_request
    }
}
