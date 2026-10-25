use super::{ReadModelWatchReceiverError, ReadModelWatchSignal};

/// Receives coalesced change notifications for one watch.
/// Dropping the receiver lets the endpoint reclaim its local sender.
/// The persisted registration remains unless the watch explicitly closes it.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchReceiver: Send {
    /// Waits for an unconsumed change and returns `ReadModelWatchSignal::Changed`,
    /// consuming the pending signal.
    /// Multiple notifications may be coalesced. Notifications arriving after this method
    /// returns remain pending, including while the caller executes a query.
    ///
    /// Cancelling a pending call must not consume a notification. Returns
    /// `ReadModelWatchSignal::Closed` when reception ends; subsequent calls also return
    /// `ReadModelWatchSignal::Closed`.
    async fn receive(&mut self) -> Result<ReadModelWatchSignal, ReadModelWatchReceiverError>;
}
