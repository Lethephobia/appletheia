use super::OutboxRelayConfig;

pub trait OutboxRelayConfigAccess {
    fn config(&self) -> &OutboxRelayConfig;
}
