use std::fmt::Debug;

use crate::unit_of_work::SnapshotPolicy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryConfig {
    pub snapshot_policy: SnapshotPolicy,
}
