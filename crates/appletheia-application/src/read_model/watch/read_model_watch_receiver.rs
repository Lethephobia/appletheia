use super::{ReadModelWatchReceiverError, ReadModelWatchSignal};

/// Receives coalesced change notifications for one watch.
/// Dropping the receiver stops renewal of the watch's lease; immediate registry removal
/// requires an explicit call to the endpoint's `remove_watch` method.
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
