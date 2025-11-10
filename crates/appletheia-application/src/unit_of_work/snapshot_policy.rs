use super::snapshot_interval::SnapshotInterval;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotPolicy {
    Disabled,
    AtLeast { minimum_interval: SnapshotInterval },
}
