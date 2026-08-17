use crate::read_model::list::ReadModelListMatcher;
use crate::read_model::{ReadModel, ReadModelFragmentChangeEnvelope};

use super::typed_read_model_watch_router::TypedReadModelWatchRouter;
use super::{
    DefaultReadModelWatchChangeRouter, DefaultReadModelWatchChangeRouterError,
    ReadModelTypedListWatch, ReadModelWatchPartitionState, ReadModelWatchRoute,
};

pub(super) struct ListReadModelWatchRouter<R, M>
where
    R: ReadModel,
    M: ReadModelListMatcher,
{
    router: DefaultReadModelWatchChangeRouter<R>,
    matcher: M,
    watched_list: ReadModelTypedListWatch<M::Query, M::Cursor>,
}

impl<R, M> ListReadModelWatchRouter<R, M>
where
    R: ReadModel,
    M: ReadModelListMatcher,
{
    pub(super) fn new(
        matcher: M,
        watched_list: ReadModelTypedListWatch<M::Query, M::Cursor>,
    ) -> Self {
        Self {
            router: DefaultReadModelWatchChangeRouter::new(),
            matcher,
            watched_list,
        }
    }
}

impl<R, M> TypedReadModelWatchRouter<R> for ListReadModelWatchRouter<R, M>
where
    R: ReadModel,
    M: ReadModelListMatcher,
{
    fn route(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
        partition_state: &ReadModelWatchPartitionState,
    ) -> Result<ReadModelWatchRoute, DefaultReadModelWatchChangeRouterError> {
        self.router
            .route_list(envelope, partition_state, &self.matcher, &self.watched_list)
    }
}
