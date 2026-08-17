use crate::read_model::{ReadModel, ReadModelFragmentChangeEnvelope};

use super::{
    DefaultReadModelWatchChangeRouterError, ReadModelWatchPartitionState, ReadModelWatchRoute,
};

pub(super) trait TypedReadModelWatchRouter<R>: Send
where
    R: ReadModel,
{
    fn route(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
        partition_state: &ReadModelWatchPartitionState,
    ) -> Result<ReadModelWatchRoute, DefaultReadModelWatchChangeRouterError>;
}
