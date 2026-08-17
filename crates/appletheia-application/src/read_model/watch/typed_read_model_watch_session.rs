use std::marker::PhantomData;

use crate::read_model::{ReadModel, ReadModelFragmentChangeEnvelope};

use super::erased_read_model_watch_session::{
    ErasedReadModelWatchSession, ReadModelWatchSessionDispatchFuture,
};
use super::typed_read_model_watch_router::TypedReadModelWatchRouter;
use super::{ReadModelWatchDelivery, ReadModelWatchPartitionState, ReadModelWatchSessionId};

pub(super) struct TypedReadModelWatchSession<R, W, D>
where
    R: ReadModel,
    W: TypedReadModelWatchRouter<R>,
{
    session_id: ReadModelWatchSessionId,
    watch_router: W,
    read_model: PhantomData<fn() -> R>,
    delivery: D,
}

impl<R, W, D> TypedReadModelWatchSession<R, W, D>
where
    R: ReadModel,
    W: TypedReadModelWatchRouter<R>,
{
    pub(super) fn new(session_id: ReadModelWatchSessionId, watch_router: W, delivery: D) -> Self {
        Self {
            session_id,
            watch_router,
            read_model: PhantomData,
            delivery,
        }
    }
}

impl<R, W, D> ErasedReadModelWatchSession for TypedReadModelWatchSession<R, W, D>
where
    R: ReadModel + 'static,
    W: TypedReadModelWatchRouter<R> + 'static,
    D: ReadModelWatchDelivery,
{
    fn dispatch<'a>(
        &'a mut self,
        envelope: &'a ReadModelFragmentChangeEnvelope,
        partition_state: &'a ReadModelWatchPartitionState,
    ) -> ReadModelWatchSessionDispatchFuture<'a> {
        Box::pin(async move {
            let route = self.watch_router.route(envelope, partition_state)?;
            let should_deliver = route.change.is_some() || route.list_invalidated;
            if should_deliver {
                self.delivery.deliver(&self.session_id, &route).await?;
            }
            Ok(route)
        })
    }
}
