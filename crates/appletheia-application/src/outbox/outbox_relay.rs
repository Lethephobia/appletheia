use super::{OutboxRelayConfig, OutboxRelayError};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay {
    fn config(&self) -> &OutboxRelayConfig;

    async fn run_forever(&self) -> Result<(), OutboxRelayError>;

    async fn run_once(&self) -> Result<(), OutboxRelayError>;
}
