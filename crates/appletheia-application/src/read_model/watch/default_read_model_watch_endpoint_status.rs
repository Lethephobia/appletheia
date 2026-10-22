#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum DefaultReadModelWatchEndpointStatus {
    Idle,
    Starting,
    Running,
    Closed,
}
