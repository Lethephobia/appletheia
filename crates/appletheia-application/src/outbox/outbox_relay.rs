use super::{Outbox, OutboxRelayError, OutboxRelayRunReport};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay: Send + Sync {
    type Outbox: Outbox;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&self) -> Result<(), OutboxRelayError>;

    async fn run_once(&self) -> Result<OutboxRelayRunReport, OutboxRelayError>;
}
