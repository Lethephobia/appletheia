use appletheia::domain::AggregateId;
use banking_ledger_domain::withdrawal::WithdrawalId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PoolTokenTransferIdempotencyKey([u8; 16]);

impl PoolTokenTransferIdempotencyKey {
    pub(crate) fn into_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl From<WithdrawalId> for PoolTokenTransferIdempotencyKey {
    fn from(withdrawal_id: WithdrawalId) -> Self {
        Self(*withdrawal_id.value().as_bytes())
    }
}
