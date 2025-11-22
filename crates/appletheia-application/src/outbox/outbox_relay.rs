use super::{OutboxRelayConfigAccess, OutboxRelayError};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay: OutboxRelayConfigAccess {
    async fn run_forever(&self) -> Result<(), OutboxRelayError>;

    async fn run_once(&self) -> Result<(), OutboxRelayError>;

    fn request_graceful_stop(&mut self);

    fn is_stop_requested(&self) -> bool;
}
