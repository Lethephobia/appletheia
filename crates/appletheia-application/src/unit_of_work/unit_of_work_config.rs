use super::snapshot_policy::SnapshotPolicy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitOfWorkConfig {
    pub snapshot_policy: SnapshotPolicy,
}
