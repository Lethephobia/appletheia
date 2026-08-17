use crate::read_model::ReadModel;
use crate::read_model::list::ReadModelListMatcher;

use super::{ReadModelTypedListWatch, ReadModelWatchDelivery, ReadModelWatchSessionId};

/// Opens typed read-model watch sessions for active client connections.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchSessionOpener: Send + Sync {
    async fn open_snapshot<R, D>(&self, delivery: D) -> ReadModelWatchSessionId
    where
        R: ReadModel + 'static,
        D: ReadModelWatchDelivery;

    async fn open_list<R, M, D>(
        &self,
        matcher: M,
        watched_list: ReadModelTypedListWatch<M::Query, M::Cursor>,
        delivery: D,
    ) -> ReadModelWatchSessionId
    where
        R: ReadModel + 'static,
        M: ReadModelListMatcher + 'static,
        D: ReadModelWatchDelivery;
}
