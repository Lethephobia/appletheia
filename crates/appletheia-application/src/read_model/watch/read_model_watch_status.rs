/// Tracks the lifecycle of a locally registered watch.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReadModelWatchStatus {
    Registered,
    Removing,
}
