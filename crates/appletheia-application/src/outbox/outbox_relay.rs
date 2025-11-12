use super::outbox_relay_config::OutboxRelayConfig;

#[allow(async_fn_in_trait)]
pub trait OutboxRelay {
    fn config(&self) -> &OutboxRelayConfig;
}
