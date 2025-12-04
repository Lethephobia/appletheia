use super::{
    OutboxFetcherProvider, OutboxPublisherAccess, OutboxRelayConfigAccess, OutboxRelayError,
    OutboxWriterProvider,
};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay:
    OutboxRelayConfigAccess + OutboxFetcherProvider + OutboxPublisherAccess + OutboxWriterProvider
{
    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&mut self) -> Result<(), OutboxRelayError>;

    async fn run_once(&mut self) -> Result<(), OutboxRelayError>;
}
