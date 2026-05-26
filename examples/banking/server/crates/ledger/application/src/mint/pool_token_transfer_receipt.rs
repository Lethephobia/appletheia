use super::OnchainTransactionId;

/// Returned after an external pool token transfer succeeds.
#[derive(Clone, Debug, PartialEq, Eq)]
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
}
