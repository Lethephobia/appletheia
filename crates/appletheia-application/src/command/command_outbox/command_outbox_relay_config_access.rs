use super::CommandOutboxRelayConfig;

pub trait CommandOutboxRelayConfigAccess {
    fn config(&self) -> &CommandOutboxRelayConfig;
}
