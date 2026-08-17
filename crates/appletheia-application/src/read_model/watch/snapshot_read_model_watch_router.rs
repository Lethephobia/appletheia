use crate::read_model::{ReadModel, ReadModelFragmentChangeEnvelope};

use super::typed_read_model_watch_router::TypedReadModelWatchRouter;
use super::{
    DefaultReadModelWatchChangeRouter, DefaultReadModelWatchChangeRouterError,
    ReadModelWatchPartitionState, ReadModelWatchRoute,
};

pub(super) struct SnapshotReadModelWatchRouter<R>
where
    R: ReadModel,
{
    router: DefaultReadModelWatchChangeRouter<R>,
}

impl<R> SnapshotReadModelWatchRouter<R>
where
    R: ReadModel,
{
    pub(super) fn new() -> Self {
        Self {
            router: DefaultReadModelWatchChangeRouter::new(),
        }
    }
}

impl<R> TypedReadModelWatchRouter<R> for SnapshotReadModelWatchRouter<R>
where
    R: ReadModel,
{
    fn route(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
        partition_state: &ReadModelWatchPartitionState,
    ) -> Result<ReadModelWatchRoute, DefaultReadModelWatchChangeRouterError> {
        self.router.route(envelope, partition_state)
    }
}
