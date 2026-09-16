/// Tracks whether a session accepts watches or is awaiting removal.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReadModelWatchSessionStatus {
    Open,
    Closing,
}
