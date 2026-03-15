use std::fmt::Debug;

use crate::snapshot::SnapshotPolicy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryConfig {
    pub snapshot_policy: SnapshotPolicy,
}
