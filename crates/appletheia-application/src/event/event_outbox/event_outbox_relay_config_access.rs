use super::EventOutboxRelayConfig;

pub trait EventOutboxRelayConfigAccess {
    fn config(&self) -> &EventOutboxRelayConfig;
}
