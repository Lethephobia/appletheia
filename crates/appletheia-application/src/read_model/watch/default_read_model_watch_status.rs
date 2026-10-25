#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DefaultReadModelWatchStatus {
    RefreshRequired,
    Waiting,
    Closing,
    Closed,
}
