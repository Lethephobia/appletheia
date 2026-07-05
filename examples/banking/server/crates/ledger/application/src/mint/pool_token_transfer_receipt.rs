use banking_ledger_domain::core::OnchainTransactionId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoolTokenTransferReceipt {
    onchain_transaction_id: OnchainTransactionId,
}

impl PoolTokenTransferReceipt {
    pub fn new(onchain_transaction_id: OnchainTransactionId) -> Self {
        Self {
            onchain_transaction_id,
        }
    }

    pub fn onchain_transaction_id(&self) -> &OnchainTransactionId {
        &self.onchain_transaction_id
    }

    pub fn into_onchain_transaction_id(self) -> OnchainTransactionId {
        self.onchain_transaction_id
    }
}
