use std::future::Future;
use std::pin::Pin;

use crate::read_model::ReadModelFragmentChangeEnvelope;

use super::{ReadModelWatchDispatchError, ReadModelWatchPartitionState, ReadModelWatchRoute};

pub(super) type ReadModelWatchSessionDispatchFuture<'a> = Pin<
    Box<dyn Future<Output = Result<ReadModelWatchRoute, ReadModelWatchDispatchError>> + Send + 'a>,
>;

pub(super) trait ErasedReadModelWatchSession: Send {
    fn dispatch<'a>(
        &'a mut self,
        envelope: &'a ReadModelFragmentChangeEnvelope,
        partition_state: &'a ReadModelWatchPartitionState,
    ) -> ReadModelWatchSessionDispatchFuture<'a>;
}
