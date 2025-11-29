use super::{
    OutboxFetcherProvider, OutboxPublisherProvider, OutboxRelayConfigAccess, OutboxRelayError,
};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay:
    OutboxRelayConfigAccess + OutboxFetcherProvider + OutboxPublisherProvider
{
    async fn run_forever(&mut self) -> Result<(), OutboxRelayError>;

    async fn run_once(&mut self) -> Result<(), OutboxRelayError>;

    fn request_graceful_stop(&mut self);

    fn is_stop_requested(&self) -> bool;
}
