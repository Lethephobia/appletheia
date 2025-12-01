use super::{
    Outbox, OutboxAckerError, OutboxBatchSize, OutboxDispatchError, OutboxLeaseDuration,
    OutboxRelayInstance,
};

#[allow(async_fn_in_trait)]
pub trait OutboxAcker {
    async fn ack(&mut self, outbox: &Outbox) -> Result<(), OutboxAckerError>;

    async fn nack(
        &mut self,
        outbox: &Outbox,
        cause: &OutboxDispatchError,
    ) -> Result<(), OutboxAckerError>;

    async fn renew_leases(
        &mut self,
        outboxes: &[Outbox],
        owner: &OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<(), OutboxAckerError>;

    async fn release_expired(&mut self, limit: OutboxBatchSize) -> Result<(), OutboxAckerError>;
}
