use super::snapshot_policy::SnapshotPolicy;
use crate::request_context::RequestContext;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitOfWorkConfig {
    pub request_context: RequestContext,
    pub snapshot_policy: SnapshotPolicy,
}
